//! Deadlock on this PC, as the backend sees it: whether it runs, where Steam has it, the recycle bin.

use crate::backend::Game;
use deadloader_core::fs::Disk;
use std::path::PathBuf;

/// Deadlock on this PC, found through Steam.
pub struct SteamGame;

impl Game for SteamGame {
    fn running(&self) -> bool {
        // If the process list can't be read, assume the game runs: waiting is safe, writing into a running game's
        // folder is not.
        deadloader_platform_windows::process::game_running().unwrap_or(true)
    }

    fn launch(&self) -> std::io::Result<()> {
        deadloader_platform_windows::shell::open_url(
            deadloader_platform_windows::shell::DEADLOCK_RUN_URL,
        )
    }

    fn move_to_recycle_bin(&self, path: &std::path::Path) -> std::io::Result<()> {
        deadloader_platform_windows::recycle::move_to_recycle_bin(path)
    }

    fn detected(&self) -> Vec<deadloader_core::session::Detected> {
        match deadloader_platform_windows::steam::steam_root() {
            Ok(Some(root)) => deadloader_core::steam::find_installations(&root, &Disk)
                .installations
                .into_iter()
                .map(|i| deadloader_core::session::Detected {
                    game_root: i.game_root,
                    library: Some(i.library),
                    build_id: i.build_id,
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    fn installations(&self) -> Vec<PathBuf> {
        match deadloader_platform_windows::steam::steam_root() {
            Ok(Some(root)) => deadloader_core::steam::find_installations(&root, &Disk)
                .installations
                .into_iter()
                .map(|i| i.game_root)
                .collect(),
            _ => Vec::new(),
        }
    }
}
