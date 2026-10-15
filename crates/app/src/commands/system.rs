//! The push channel and what Windows tells about the user (languages, regional format).

use super::*;

/// The UI's channel for pushes (download progress, the game starting and closing, dropped files, start notices).
/// Called once after the page loads, and again after a reload; the newest channel wins.
#[tauri::command]
pub fn subscribe(outbox: State<'_, Arc<Outbox>>, events: Channel<UiEvent>) {
    outbox.connect(Box::new(ChannelSink(events)));
}

struct ChannelSink(Channel<UiEvent>);

impl Sink for ChannelSink {
    fn deliver(&self, event: &UiEvent) -> bool {
        self.0.send(event.clone()).is_ok()
    }
}

/// The user's Windows languages, best first (the "system" UI language, 179), and the regional format of times and
/// numbers (178). Empty or `None` when Windows can't tell; the UI then uses English and its own defaults.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SystemLocale {
    pub languages: Vec<String>,
    pub region: Option<String>,
}

#[tauri::command]
pub fn system_locale() -> SystemLocale {
    SystemLocale {
        languages: deadloader_platform_windows::locale::preferred_ui_languages()
            .unwrap_or_default(),
        region: deadloader_platform_windows::locale::regional_format().ok(),
    }
}
