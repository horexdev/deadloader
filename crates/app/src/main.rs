//! deadloader: one window, no tray; closing the window ends the process (decision 95).

// Release builds are GUI programs without a console window.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod activity;
mod backend;
mod catalog;
mod commands;
mod downloads;
mod events;
mod game;
mod network;
mod paths;
#[cfg(test)]
mod policy;
mod progress;
mod protocol;
mod push;
mod setup;
mod startup;
mod updates;
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
