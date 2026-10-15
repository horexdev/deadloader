//! Start-up: the data folder, single instance, the log, and the main window with our WebView2 settings.

use crate::backend::Backend;
use crate::commands;
use crate::paths::{DataDirError, DataLayout};
use crate::webview;
use deadloader_core::log::{DiskLogDir, Level, Logger, SystemClock};
use serde_json::{Map, Value};
use std::fmt;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use tauri::webview::PageLoadEvent;
use tauri::{App, AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

const MAIN_WINDOW: &str = "main";

type Log = Arc<Logger<SystemClock, DiskLogDir>>;

#[derive(Debug)]
pub enum StartError {
    /// The path of the running executable is unknown.
    Exe(io::Error),
    DataDir(DataDirError),
    /// Tauri or WebView2 failed to start.
    Window(tauri::Error),
}

impl fmt::Display for StartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exe(e) => write!(
                f,
                "Windows did not tell deadloader where deadloader.exe is ({e}), so it cannot find its data folder. \
                 Start deadloader.exe directly from its folder."
            ),
            Self::DataDir(e) => e.fmt(f),
            Self::Window(e) => write!(
                f,
                "The window could not be opened: {e}. deadloader draws its window with Microsoft WebView2, which is \
                 part of Windows 10 and 11. If it was removed, install the \"Microsoft Edge WebView2 Runtime\" from \
                 Microsoft and start deadloader again; the log in deadloader-data\\logs has the details."
            ),
        }
    }
}

impl std::error::Error for StartError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Exe(e) => Some(e),
            Self::DataDir(e) => Some(e),
            Self::Window(e) => Some(e),
        }
    }
}

pub fn run() -> Result<(), StartError> {
    let started = Instant::now();
    let exe = std::env::current_exe().map_err(StartError::Exe)?;
    let layout = DataLayout::beside(&exe).map_err(StartError::DataDir)?;
    layout.ensure_writable().map_err(StartError::DataDir)?;

    // Decision 92: a second launch hands over to the running instance and exits inside `build`, before it logs or
    // opens anything.
    let app = crate::protocol::register(tauri::Builder::default())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            focus_main_window(app);
        }))
        .invoke_handler(tauri::generate_handler![
            commands::library,
            commands::set_enabled,
            commands::move_mod,
            commands::switch_variant,
            commands::choose,
            commands::select_profile,
            commands::create_profile,
            commands::profile_names,
            commands::import_paths,
            commands::apply,
        ])
        .build(tauri::generate_context!())
        .map_err(StartError::Window)?;

    let log: Log = Arc::new(Logger::open(layout.logs()));
    app.manage(Arc::clone(&log));
    log_start(&log, &layout);
    // Only after `build`: a second instance has exited by now, so it never touches the journal or the library.
    app.manage(Arc::new(Backend::start(
        layout.root(),
        Box::new(commands::SteamGame),
        Arc::clone(&log) as deadloader_core::session::SharedLog,
    )));
    if let Err(e) = open_main_window(&app, &layout, &log, started) {
        write(
            &log,
            Level::Error,
            "main window could not be opened",
            [("error", e.to_string().into())],
        );
        return Err(StartError::Window(e));
    }
    app.run(|_, _| {});
    Ok(())
}

fn open_main_window(
    app: &App,
    layout: &DataLayout,
    log: &Log,
    started: Instant,
) -> tauri::Result<()> {
    let loaded = AtomicBool::new(false);
    let on_load_log = Arc::clone(log);
    let window = WebviewWindowBuilder::new(app, MAIN_WINDOW, WebviewUrl::App("index.html".into()))
        .title("deadloader")
        // Designed at 1280 × 800; the minimum is the design's 960 × 600.
        .inner_size(1280.0, 800.0)
        .min_inner_size(960.0, 600.0)
        // `--bg-app`, so the window is not white before the page paints.
        .background_color(tauri::window::Color(0x14, 0x12, 0x18, 0xff))
        .additional_browser_args(&webview::browser_args())
        .data_directory(layout.webview())
        // A secure origin (`https://tauri.localhost`): plain-http subresources are blocked as mixed content.
        .use_https_scheme(true)
        .on_page_load(move |_, payload| {
            if payload.event() == PageLoadEvent::Finished && !loaded.swap(true, Ordering::Relaxed) {
                let ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
                write(
                    &on_load_log,
                    Level::Info,
                    "page loaded",
                    [("ms_since_start", ms.into())],
                );
            }
        })
        .build()?;

    let expected = layout.webview();
    let check_log = Arc::clone(log);
    window.with_webview(move |platform| {
        check_profile_folder(&check_log, &expected, platform);
    })
}

#[cfg(windows)]
fn check_profile_folder(
    log: &Log,
    expected: &std::path::Path,
    platform: tauri::webview::PlatformWebview,
) {
    match deadloader_platform_windows::webview2::user_data_folder(&platform.controller()) {
        Ok(actual) if webview::same_folder(expected, &actual) => write(
            log,
            Level::Info,
            "webview2 profile folder",
            [("folder", path_value(&actual))],
        ),
        Ok(actual) => write(
            log,
            Level::Warn,
            "webview2 profile folder is not where deadloader put it",
            [
                ("folder", path_value(&actual)),
                ("expected", path_value(expected)),
            ],
        ),
        Err(e) => write(
            log,
            Level::Warn,
            "could not read the webview2 profile folder",
            [("error", e.to_string().into())],
        ),
    }
}

#[cfg(not(windows))]
fn check_profile_folder(_: &Log, _: &std::path::Path, _: tauri::webview::PlatformWebview) {}

fn focus_main_window(app: &AppHandle) {
    if let Some(log) = app.try_state::<Log>() {
        write(&log, Level::Info, "second launch: focusing this window", []);
    }
    if let Some(window) = app.get_webview_window(MAIN_WINDOW) {
        // Best effort: Windows may refuse to bring a window forward; the taskbar button flashes then.
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

/// What ADR 0001 asks to log at start: the runtime version, every `WEBVIEW2_*` variable, the profile folder.
fn log_start(log: &Log, layout: &DataLayout) {
    write(
        log,
        Level::Info,
        "deadloader started",
        [
            ("version", env!("CARGO_PKG_VERSION").into()),
            ("data_folder", path_value(layout.root())),
        ],
    );
    match tauri::webview_version() {
        Ok(version) => write(
            log,
            Level::Info,
            "webview2 runtime",
            [("version", version.into())],
        ),
        Err(e) => write(
            log,
            Level::Error,
            "webview2 runtime not found",
            [("error", e.to_string().into())],
        ),
    }
    let variables: Map<String, Value> = webview::webview2_variables(std::env::vars_os())
        .into_iter()
        .map(|(name, value)| (name, value.into()))
        .collect();
    let lvl = if variables.is_empty() {
        Level::Info
    } else {
        Level::Warn
    };
    write(
        log,
        lvl,
        "webview2 environment variables",
        [("variables", variables.into())],
    );
    write(
        log,
        Level::Info,
        "webview2 settings",
        [
            ("profile_folder", path_value(&layout.webview())),
            ("browser_args", webview::browser_args().into()),
        ],
    );
}

fn path_value(path: &std::path::Path) -> Value {
    PathBuf::from(path).display().to_string().into()
}

/// A failure to log has nowhere else to go; it must not stop the app.
fn write<const N: usize>(log: &Log, lvl: Level, msg: &str, fields: [(&str, Value); N]) {
    let fields: Map<String, Value> = fields.into_iter().map(|(k, v)| (k.to_owned(), v)).collect();
    let _ = log.app(lvl, msg, fields);
}
