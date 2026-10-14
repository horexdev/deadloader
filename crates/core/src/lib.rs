//! Domain logic of deadloader: library, profiles, VPK parsing, the gameinfo.gi guard, planning of enable/disable
//! operations, the intent journal, conflict detection and the manifest. No network code; the filesystem, processes and
//! the clock are reached through traits so everything here is testable without a game install.

pub mod vdf;
