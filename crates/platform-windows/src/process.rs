//! Running processes, to know whether Deadlock is running (changes wait until it exits, decision 17).
//!
//! Uses a ToolHelp snapshot of the process list: read-only, no access to other processes is opened. This module is
//! the one place in deadloader that needs `unsafe` (Win32 FFI); every call states why it is sound.

#![allow(unsafe_code)]

use std::io;
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};

pub const GAME_EXE: &str = "deadlock.exe";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessInfo {
    pub pid: u32,
    /// May name a process that has exited and whose id was reused; compare creation times before trusting it.
    pub parent_pid: u32,
    pub exe_name: String,
}

pub fn game_running() -> io::Result<bool> {
    Ok(find(&list_processes()?, GAME_EXE).is_some())
}

/// The first process whose executable name equals `exe_name`, ignoring case.
pub fn find<'a>(processes: &'a [ProcessInfo], exe_name: &str) -> Option<&'a ProcessInfo> {
    processes
        .iter()
        .find(|p| p.exe_name.eq_ignore_ascii_case(exe_name))
}

pub fn list_processes() -> io::Result<Vec<ProcessInfo>> {
    // SAFETY: plain call with constant arguments; the result is checked before use.
    let handle = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if handle == INVALID_HANDLE_VALUE {
        return Err(io::Error::last_os_error());
    }
    let snapshot = Snapshot(handle);
    let mut entry = PROCESSENTRY32W {
        dwSize: size_of::<PROCESSENTRY32W>() as u32,
        ..Default::default()
    };
    let mut out = Vec::new();
    // SAFETY: `snapshot.0` is a valid snapshot handle until `snapshot` drops; `entry` is a properly sized,
    // writable PROCESSENTRY32W with `dwSize` set, as the API requires.
    let mut ok = unsafe { Process32FirstW(snapshot.0, &mut entry) };
    while ok != 0 {
        let len = entry
            .szExeFile
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(entry.szExeFile.len());
        out.push(ProcessInfo {
            pid: entry.th32ProcessID,
            parent_pid: entry.th32ParentProcessID,
            exe_name: String::from_utf16_lossy(&entry.szExeFile[..len]),
        });
        // SAFETY: as for Process32FirstW above.
        ok = unsafe { Process32NextW(snapshot.0, &mut entry) };
    }
    Ok(out)
}

struct Snapshot(HANDLE);

impl Drop for Snapshot {
    fn drop(&mut self) {
        // SAFETY: the handle came from a successful CreateToolhelp32Snapshot and is closed exactly once.
        unsafe { CloseHandle(self.0) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn proc(pid: u32, name: &str) -> ProcessInfo {
        ProcessInfo {
            pid,
            parent_pid: 0,
            exe_name: name.to_owned(),
        }
    }

    #[test]
    fn finds_a_process_by_exe_name_ignoring_case() {
        let list = [proc(1, "explorer.exe"), proc(42, "Deadlock.EXE")];
        assert_eq!(find(&list, GAME_EXE).map(|p| p.pid), Some(42));
    }

    #[test]
    fn similar_names_are_not_the_game() {
        let list = [
            proc(1, "notdeadlock.exe"),
            proc(2, "deadlock.exe.bak"),
            proc(3, "deadlock"),
        ];
        assert!(find(&list, GAME_EXE).is_none());
    }

    #[test]
    fn the_real_process_list_contains_this_test() {
        let list = list_processes().expect("snapshot");
        let me = list
            .iter()
            .find(|p| p.pid == std::process::id())
            .expect("own process listed");
        assert!(me.exe_name.to_ascii_lowercase().ends_with(".exe"), "{me:?}");
        assert!(
            list.len() > 5,
            "a real system runs more than a handful of processes"
        );
    }

    #[test]
    fn checking_for_the_game_works_on_a_real_system() {
        // Deadlock may or may not run on this machine; the call itself must succeed.
        game_running().expect("process list");
    }
}
