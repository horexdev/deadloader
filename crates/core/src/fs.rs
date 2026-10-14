//! The file access the core needs, as traits, so logic can be tested with in-memory fakes.

use std::io;
use std::path::Path;

pub trait ReadText {
    /// The whole file as UTF-8 text.
    fn read_text(&self, path: &Path) -> io::Result<String>;
}

/// The real filesystem.
#[derive(Debug, Default, Clone, Copy)]
pub struct Disk;

impl ReadText for Disk {
    fn read_text(&self, path: &Path) -> io::Result<String> {
        std::fs::read_to_string(path)
    }
}
