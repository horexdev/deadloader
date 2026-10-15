//! Where downloads report what they did: file operations to the files log, decisions (a restart from zero, a failure)
//! to the app log. Requests are logged by the network client itself.

use deadloader_core::log::{Clock, FileEvent, FileOp, Level, LogDir, Logger, Outcome};
use serde_json::{Map, Value};
use std::io;
use std::path::Path;

pub trait DownloadLog: Send + Sync {
    fn file(&self, msg: &str, event: &FileEvent);
    /// Returns the line's id, so a user-facing error can point at its details.
    fn app(&self, lvl: Level, msg: &str, fields: Map<String, Value>) -> Option<String>;
}

impl<C, D> DownloadLog for Logger<C, D>
where
    C: Clock + Send + Sync,
    D: LogDir + Send + Sync,
{
    fn file(&self, msg: &str, event: &FileEvent) {
        // A download must not fail because a log line could not be written.
        let _ = Logger::file(self, msg, event);
    }

    fn app(&self, lvl: Level, msg: &str, fields: Map<String, Value>) -> Option<String> {
        Logger::app(self, lvl, msg, fields)
            .ok()
            .map(|id| id.as_str().to_owned())
    }
}

/// Logs one file operation with its result and hands the result back.
pub(crate) fn file_op<T>(
    log: &dyn DownloadLog,
    op: FileOp,
    msg: &str,
    src: Option<&Path>,
    dst: Option<&Path>,
    bytes: Option<u64>,
    result: io::Result<T>,
) -> io::Result<T> {
    let event = FileEvent {
        src: src.map(|p| p.display().to_string()),
        dst: dst.map(|p| p.display().to_string()),
        bytes,
        result: if result.is_ok() {
            Outcome::Ok
        } else {
            Outcome::Failed
        },
        error: result.as_ref().err().map(ToString::to_string),
        ..FileEvent::new(op)
    };
    log.file(msg, &event);
    result
}

pub(crate) fn fields<const N: usize>(pairs: [(&str, Value); N]) -> Map<String, Value> {
    pairs.into_iter().map(|(k, v)| (k.to_owned(), v)).collect()
}

#[cfg(test)]
pub(crate) mod mem {
    use super::*;
    use std::sync::Mutex;

    type AppLine = (Level, String, Map<String, Value>);

    /// Every line written, for assertions.
    #[derive(Debug, Default)]
    pub struct MemLog {
        pub files: Mutex<Vec<FileEvent>>,
        pub file_msgs: Mutex<Vec<String>>,
        pub app: Mutex<Vec<AppLine>>,
    }

    impl MemLog {
        pub fn files(&self) -> Vec<(String, FileEvent)> {
            let msgs = self.file_msgs.lock().expect("lock").clone();
            let events = self.files.lock().expect("lock").clone();
            msgs.into_iter().zip(events).collect()
        }

        pub fn app_msgs(&self) -> Vec<String> {
            self.app
                .lock()
                .expect("lock")
                .iter()
                .map(|(_, m, _)| m.clone())
                .collect()
        }
    }

    impl DownloadLog for MemLog {
        fn file(&self, msg: &str, event: &FileEvent) {
            self.file_msgs.lock().expect("lock").push(msg.to_owned());
            self.files.lock().expect("lock").push(event.clone());
        }

        fn app(&self, lvl: Level, msg: &str, fields: Map<String, Value>) -> Option<String> {
            let mut app = self.app.lock().expect("lock");
            app.push((lvl, msg.to_owned(), fields));
            Some(format!("line-{}", app.len()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deadloader_core::fs::DirEntry;
    use deadloader_core::log::Masker;
    use std::sync::Mutex;
    use time::OffsetDateTime;

    #[derive(Default)]
    struct Lines(Mutex<Vec<(String, String)>>);

    impl LogDir for Lines {
        fn append_line(&self, name: &str, line: &str) -> io::Result<()> {
            self.0
                .lock()
                .expect("lock")
                .push((name.to_owned(), line.to_owned()));
            Ok(())
        }
        fn list(&self) -> io::Result<Vec<DirEntry>> {
            Ok(Vec::new())
        }
        fn remove(&self, _: &str) -> io::Result<()> {
            Ok(())
        }
        fn read(&self, _: &str) -> io::Result<Vec<u8>> {
            Ok(Vec::new())
        }
    }

    struct Fixed;
    impl Clock for Fixed {
        fn now(&self) -> OffsetDateTime {
            OffsetDateTime::UNIX_EPOCH
        }
    }

    #[test]
    fn the_core_logger_writes_file_and_app_lines() {
        let logger = Logger::new(Fixed, Lines::default(), Masker::none(), Default::default());
        let log: &dyn DownloadLog = &logger;
        let r = file_op(
            log,
            FileOp::Delete,
            "deleted download part",
            Some(Path::new("x.part")),
            None,
            Some(3),
            Ok(()),
        );
        assert!(r.is_ok());
        let id = log.app(Level::Warn, "restarted", fields([("task", 1.into())]));
        assert!(id.is_some());
        let lines = logger.dir().0.lock().expect("lock").clone();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].0.starts_with("files-"), "{lines:?}");
        assert!(lines[0].1.contains("\"op\":\"delete\""), "{lines:?}");
        assert!(lines[1].0.starts_with("app-"), "{lines:?}");
    }

    #[test]
    fn failures_are_logged_with_the_error() {
        let log = mem::MemLog::default();
        let r: io::Result<()> = file_op(
            &log,
            FileOp::Rename,
            "renamed",
            Some(Path::new("a")),
            Some(Path::new("b")),
            None,
            Err(io::Error::from(io::ErrorKind::PermissionDenied)),
        );
        assert!(r.is_err());
        let (_, event) = &log.files()[0];
        assert_eq!(event.result, Outcome::Failed);
        assert!(event.error.is_some());
    }
}
