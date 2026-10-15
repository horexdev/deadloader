//! deadloader: one window, no tray; closing the window ends the process (decision 95).

// Release builds are GUI programs without a console window.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod backend;
mod commands;
mod paths;
#[cfg(test)]
mod policy;
mod protocol;
mod startup;
mod webview;

use std::process::ExitCode;

fn main() -> ExitCode {
    match startup::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            // The proper error screen comes later; until then the console and the log carry the message.
            eprintln!("deadloader: {error}");
            ExitCode::FAILURE
        }
    }
}
