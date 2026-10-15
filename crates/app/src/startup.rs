//! Start-up: the data folder, single instance, the log, and the main window with our WebView2 settings.

use crate::backend::Backend;
use crate::catalog::{self, CatalogSlot};
use crate::commands;
use crate::events::{Outbox, UiEvent};
use crate::network::{Network, NetworkLog};
use crate::paths::{DataDirError, DataLayout};
use crate::push::{self, GameWatch};
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
use tauri::{
    App, AppHandle, DragDropEvent, Manager, WebviewUrl, WebviewWindowBuilder, Window, WindowEvent,
};

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
    // D8: a deletion of all data that couldn't finish at the last exit is finished before anything opens a file there.
    let finished_wipe = deadloader_core::wipe::finish_pending(layout.root());
    // E1 (decision 57): without a writable data folder the window opens anyway, to say what to do, and nothing else
    // runs.
    let blocked = match layout.ensure_writable() {
        Ok(()) => None,
        Err(DataDirError::NotWritable { dir, error }) => Some(crate::setup::Startup::no_write(
            exe.parent().unwrap_or(&exe),
            &dir,
            &error,
        )),
        Err(e) => return Err(StartError::DataDir(e)),
    };

    // Decision 92: a second launch hands over to the running instance and exits inside `build`, before it logs or
    // opens anything.
    let app = crate::protocol::register(tauri::Builder::default())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            focus_main_window(app);
        }))
        .on_window_event(on_window_event)
        .invoke_handler(tauri::generate_handler![
            commands::system::subscribe,
            commands::shell::play,
            commands::shell::apply_preview,
            commands::shell::game_status,
            commands::shell::window_minimize,
            commands::shell::window_toggle_maximize,
            commands::shell::window_close,
            commands::shell::window_start_drag,
            commands::system::system_locale,
            commands::library::library,
            commands::library::set_enabled,
            commands::library::move_mod,
            commands::library::switch_variant,
            commands::library::choose,
            commands::profiles::select_profile,
            commands::profiles::create_profile,
            commands::profiles::profile_names,
            commands::library::import_paths,
            commands::library::apply,
            commands::library::removal_plan,
            commands::library::remove,
            commands::profiles::rename_profile,
            commands::profiles::duplicate_profile,
            commands::profiles::delete_profile,
            commands::profiles::export_profile,
            commands::profiles::import_profile,
            commands::settings::settings,
            commands::settings::update_settings,
            commands::library::add_from_disk,
            commands::profiles::export_profile_dialog,
            commands::profiles::import_profile_dialog,
            commands::profiles::profiles_overview,
            commands::profiles::set_profile_installation,
            commands::profiles::profile_download_missing,
            commands::catalog::catalog_list,
            commands::catalog::catalog_refresh,
            commands::catalog::catalog_prefetch,
            commands::catalog::catalog_categories,
            commands::catalog::catalog_details,
            commands::catalog::catalog_status,
            commands::catalog::catalog_retry_now,
            commands::catalog::catalog_activate,
            commands::catalog::catalog_consent_prompt,
            commands::catalog::catalog_grant,
            commands::catalog::favourites,
            commands::catalog::set_favourite,
            commands::catalog::confirm_age,
            commands::catalog::open_link,
            commands::downloads::download_install,
            commands::downloads::downloads,
            commands::downloads::download_control,
            commands::updates::updates_check,
            commands::updates::updates_dismiss,
            commands::updates::update_install,
            commands::updates::update_map,
            commands::library::mod_link,
            commands::library::mod_link_files,
            commands::library::mod_rename,
            commands::library::pending,
            commands::library::pending_revert,
            commands::library::pending_discard,
            commands::library::pending_undo_applied,
            commands::library::foreign_files,
            commands::library::foreign_leave,
            commands::library::foreign_take,
            commands::library::gameinfo_status,
            commands::library::verify_game_files,
            commands::library::show_place,
            commands::settings::settings_reset,
            commands::settings::settings_file_text,
            commands::settings::settings_file_save,
            commands::settings::cache_usage,
            commands::settings::cache_clear,
            commands::settings::installations,
            commands::settings::game_folder_dialog,
            commands::settings::installation_remove,
            commands::settings::open_folder,
            commands::settings::about,
            commands::settings::rollback_preview,
            commands::settings::rollback,
            commands::settings::wipe_summary,
            commands::settings::delete_all_data,
            commands::settings::setup_check,
            commands::settings::acknowledge_notice,
            commands::settings::choose_catalog,
            commands::settings::startup_state,
            commands::settings::blocked_open_folder,
            commands::activity::log_page,
            commands::activity::log_entry,
            commands::activity::log_counts,
            commands::activity::log_summary,
            commands::activity::open_logs_folder,
        ])
        .build(tauri::generate_context!())
        .map_err(StartError::Window)?;
    if let Some(state) = blocked {
        return run_blocked(app, state);
    }
    app.manage(crate::setup::Startup::Ready);

    let log: Log = Arc::new(Logger::open(layout.logs()));
    app.manage(Arc::clone(&log));
    log_start(&log, &layout);
    match &finished_wipe {
        Ok(Some(deleted)) => write(
            &log,
            Level::Info,
            "a deletion of all data left from the last exit was finished",
            [
                ("files", deleted.files.into()),
                ("bytes", deleted.bytes.into()),
            ],
        ),
        Ok(None) => {}
        Err(e) => write(
            &log,
            Level::Warn,
            "a deletion of all data left from the last exit could not finish",
            [("error", e.to_string().into())],
        ),
    }
    let places = crate::setup::Places {
        exe: exe.clone(),
        data: layout.root().to_path_buf(),
    };
    app.manage(places.clone());
    let wipe = Arc::new(crate::setup::WipeOnExit::default());
    app.manage(Arc::clone(&wipe));
    // Only after `build`: a second instance has exited by now, so it never touches the journal or the library.
    let backend = Arc::new(Backend::start(
        layout.root(),
        Box::new(crate::game::SteamGame),
        Arc::clone(&log) as deadloader_core::session::SharedLog,
    ));
    let outbox = Arc::new(Outbox::new());
    wire_activity(&app, &log, &layout, &backend, &outbox);
    // Nothing is requested until the catalog or a download asks; local mode until the first-run choice (108).
    let network = Arc::new(Network::new(
        &backend.settings().unwrap_or_default(),
        Arc::clone(&log) as Arc<dyn NetworkLog>,
    ));
    let follow = Arc::clone(&network);
    backend.on_settings(Box::new(move |settings| follow.apply(settings)));
    crate::network::relay_running(network.client(), Arc::clone(&outbox));
    let progress_outbox = Arc::clone(&outbox);
    backend.on_apply_progress(Box::new(move |applying| {
        progress_outbox.send_live(UiEvent::Applying(*applying));
    }));
    let settings = backend.settings().unwrap_or_default();
    let queue = crate::downloads::open(
        layout.root().join("downloads"),
        &settings,
        Arc::clone(network.client()),
        Arc::clone(&log) as Arc<dyn deadloader_downloads::DownloadLog>,
        &backend,
        Arc::clone(&outbox),
    );
    let downloads = match queue {
        Ok(downloads) => Some(Arc::new(downloads)),
        Err(message) => {
            write(
                &log,
                Level::Error,
                "the download queue could not be opened",
                [(
                    "message",
                    serde_json::to_value(&message).unwrap_or_default(),
                )],
            );
            None
        }
    };
    if let Some(downloads) = &downloads {
        backend.on_settings(crate::downloads::follow_settings(
            Arc::clone(downloads),
            &settings,
        ));
        app.manage(Arc::clone(downloads));
    }
    let slot = Arc::new(CatalogSlot::new());
    app.manage(Arc::clone(&slot));
    let catalog_parts = CatalogParts {
        client: Arc::clone(network.client()),
        backend: Arc::clone(&backend),
        outbox: Arc::clone(&outbox),
        cache_dir: layout.root().join("cache"),
    };
    app.manage(network);
    app.manage(Arc::clone(&backend));
    app.manage(Arc::clone(&outbox));
    push::start_notices(&backend, &outbox);
    let settings_source = Arc::clone(&backend);
    watch_game(backend, outbox, Arc::clone(&log));
    let window = match open_main_window(&app, &layout, &log, started) {
        Ok(window) => window,
        Err(e) => {
            write(
                &log,
                Level::Error,
                "main window could not be opened",
                [("error", e.to_string().into())],
            );
            return Err(StartError::Window(e));
        }
    };
    follow_zoom(&window, &settings_source, &log);
    open_catalog(app.handle().clone(), slot, catalog_parts, Arc::clone(&log));
    app.run(move |_, event| {
        // Running downloads save their position and stay queued for the next start.
        if let tauri::RunEvent::Exit = event
            && let Some(downloads) = &downloads
        {
            downloads.shutdown();
        }
        // D8: the data folder goes last, once the window engine has let go of its files (up to 10 s); what stays
        // locked is finished at the next start.
        if let tauri::RunEvent::Exit = event
            && wipe.armed()
        {
            crate::setup::wipe_at_exit(&places.data, 40, std::time::Duration::from_millis(250));
        }
    });
    Ok(())
}

/// Activity (S7): the log reader for its commands, the Logs settings on the logger, live pushes of new lines.
fn wire_activity(
    app: &App,
    log: &Log,
    layout: &DataLayout,
    backend: &Backend,
    outbox: &Arc<Outbox>,
) {
    crate::activity::apply_settings(log, &backend.settings().unwrap_or_default());
    let follow = Arc::clone(log);
    backend.on_settings(Box::new(move |settings| {
        crate::activity::apply_settings(&follow, settings);
    }));
    app.manage(Arc::new(crate::activity::Activity::new(
        Arc::clone(log),
        layout.logs(),
        deadloader_core::log::Masker::from_env(),
    )) as commands::activity::SharedActivity);
    if let Err(e) = crate::activity::watch(log, Arc::clone(outbox)) {
        // Activity then shows new lines when it is opened again, not live.
        write(
            log,
            Level::Error,
            "the activity view's live updates could not start",
            [("error", e.to_string().into())],
        );
    }
}

/// What the catalog is opened with.
struct CatalogParts {
    client: Arc<deadloader_net::Client>,
    backend: Arc<Backend>,
    outbox: Arc<Outbox>,
    cache_dir: PathBuf,
}

/// Opens the cache and the catalog after the window, on their own thread: listing the image folders must not delay the
/// first frame. Commands wait for it; pictures answer 503 until then.
fn open_catalog(app: AppHandle, slot: Arc<CatalogSlot>, parts: CatalogParts, log: Log) {
    let thread_log = Arc::clone(&log);
    let spawned = std::thread::Builder::new()
        .name("catalog-open".to_owned())
        .spawn(move || {
            let log = thread_log;
            let backend = parts.backend;
            let opened = catalog::open(
                &parts.cache_dir,
                &backend.settings().unwrap_or_default(),
                parts.client,
                Arc::clone(&log) as deadloader_core::session::SharedLog,
                Arc::clone(&log) as Arc<dyn deadloader_catalog::AppLog>,
                &backend,
                Arc::clone(&parts.outbox),
            );
            let outbox = parts.outbox;
            match &opened {
                Ok(catalog) => {
                    let follow = catalog.clone();
                    backend.on_settings(Box::new(move |settings| follow.apply_settings(settings)));
                    // A change saved while the catalog was opening.
                    if let Ok(settings) = backend.settings() {
                        catalog.apply_settings(&settings);
                    }
                    app.manage(crate::protocol::Images(Arc::new(catalog.clone())));
                }
                Err(message) => write(
                    &log,
                    Level::Error,
                    "the catalog could not be opened",
                    [("message", serde_json::to_value(message).unwrap_or_default())],
                ),
            }
            // Open for the UI first: the update check can take a while with many mods.
            slot.set(opened.clone());
            if let Ok(catalog) = &opened {
                check_updates_at_start(&backend, catalog, &outbox, &log);
            }
        });
    if let Err(e) = spawned {
        write(
            &log,
            Level::Error,
            "the catalog could not be opened",
            [("error", e.to_string().into())],
        );
    }
}

/// Decision 38: the automatic check at every start, unless it is turned off or the catalog is (local mode).
fn check_updates_at_start(
    backend: &Backend,
    catalog: &catalog::AppCatalog,
    outbox: &Outbox,
    log: &Log,
) {
    let Ok(settings) = backend.settings() else {
        return;
    };
    if !settings.library.check_updates_at_start || settings.is_local_mode() {
        return;
    }
    let Ok(manifest) = backend.manifest() else {
        return;
    };
    let report = crate::updates::check_all(&manifest, catalog, deadloader_catalog::Initiator::Auto);
    write(
        log,
        Level::Info,
        "mod update check at start",
        [
            ("mods", report.checks.len().into()),
            ("updates_available", report.available.into()),
            ("stopped", report.stopped.is_some().into()),
        ],
    );
    outbox.send(UiEvent::Updates(report));
}

/// Decision 143's watcher: looks for the game every [`push::GAME_POLL`] on its own thread, for the life of the process.
fn watch_game(backend: Arc<Backend>, outbox: Arc<Outbox>, log: Log) {
    let thread_log = Arc::clone(&log);
    let spawned = std::thread::Builder::new()
        .name("game-watch".to_owned())
        .spawn(move || {
            let mut watch = GameWatch::new();
            loop {
                if let Some(running) = watch.tick(&backend, &outbox) {
                    let msg = if running {
                        "deadlock is running"
                    } else {
                        "deadlock is not running"
                    };
                    write(&thread_log, Level::Info, msg, []);
                }
                std::thread::sleep(push::GAME_POLL);
            }
        });
    if let Err(e) = spawned {
        // Without it changes made while the game runs wait for Apply or Play instead.
        write(
            &log,
            Level::Error,
            "the game watcher could not start",
            [("error", e.to_string().into())],
        );
    }
}

/// Files dragged onto the window (the system's drag and drop, not the page's): a hover state for the UI, and an
/// import on a worker thread when they are dropped.
fn on_window_event(window: &Window, event: &WindowEvent) {
    let WindowEvent::DragDrop(drop) = event else {
        return;
    };
    let app = window.app_handle();
    let (Some(backend), Some(outbox)) = (
        app.try_state::<Arc<Backend>>(),
        app.try_state::<Arc<Outbox>>(),
    ) else {
        return;
    };
    match drop {
        DragDropEvent::Enter { .. } => outbox.send(UiEvent::DragHover { over: true }),
        DragDropEvent::Leave => outbox.send(UiEvent::DragHover { over: false }),
        DragDropEvent::Drop { paths, .. } => {
            let (backend, outbox, paths) =
                (Arc::clone(&backend), Arc::clone(&outbox), paths.clone());
            tauri::async_runtime::spawn_blocking(move || {
                push::import_dropped(&backend, &outbox, &paths);
            });
        }
        _ => {}
    }
}

/// E1: only the window, with its profile in a temporary folder that goes at exit; no log, no library, no network.
fn run_blocked(app: App, state: crate::setup::Startup) -> Result<(), StartError> {
    let profile = crate::setup::blocked_webview_dir();
    app.manage(state);
    main_window(&app, profile.clone())
        .build()
        .map_err(StartError::Window)?;
    app.run(move |_, event| {
        if let tauri::RunEvent::Exit = event {
            crate::setup::remove_at_exit(&profile, 40, std::time::Duration::from_millis(250));
        }
    });
    Ok(())
}

/// The main window as the design wants it, with the WebView2 profile in `profile`.
fn main_window(app: &App, profile: PathBuf) -> WebviewWindowBuilder<'_, tauri::Wry, App> {
    WebviewWindowBuilder::new(app, MAIN_WINDOW, WebviewUrl::App("index.html".into()))
        .title("deadloader")
        // Designed at 1280 × 800; the minimum is the design's 960 × 600.
        .inner_size(1280.0, 800.0)
        .min_inner_size(960.0, 600.0)
        // The design's own title bar (ui/src/shell/TitleBar.svelte) replaces the system frame; the window keeps its
        // shadow and resize borders.
        .decorations(false)
        .shadow(true)
        // `--bg-app`, so the window is not white before the page paints.
        .background_color(tauri::window::Color(0x14, 0x12, 0x18, 0xff))
        .additional_browser_args(&webview::browser_args())
        .data_directory(profile)
        // A secure origin (`https://tauri.localhost`): plain-http subresources are blocked as mixed content.
        .use_https_scheme(true)
        // The UI handles Ctrl + wheel / plus / minus / 0 and saves the zoom; WebView2's own keys would bypass it.
        .zoom_hotkeys_enabled(false)
}

/// The zoom of Settings → General on the window now and after every change of it (Ctrl + wheel / plus / minus / 0
/// save it too). WebView2's own zoom keys stay off, so the zoom is always the saved one.
fn follow_zoom(window: &tauri::WebviewWindow, backend: &Arc<Backend>, log: &Log) {
    let zoom = Arc::new(webview::Zoom::default());
    let apply = {
        let (window, log) = (window.clone(), Arc::clone(log));
        move |percent: u32| {
            if let Some(factor) = zoom.changed(percent)
                && let Err(e) = window.set_zoom(factor)
            {
                write(
                    &log,
                    Level::Warn,
                    "zoom could not be applied",
                    [("percent", percent.into()), ("error", e.to_string().into())],
                );
            }
        }
    };
    apply(
        backend
            .settings()
            .map_or(100, |settings| settings.general.zoom_percent),
    );
    backend.on_settings(Box::new(move |settings| {
        apply(settings.general.zoom_percent);
    }));
}

fn open_main_window(
    app: &App,
    layout: &DataLayout,
    log: &Log,
    started: Instant,
) -> tauri::Result<tauri::WebviewWindow> {
    let loaded = AtomicBool::new(false);
    let on_load_log = Arc::clone(log);
    let window = main_window(app, layout.webview())
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
    })?;
    Ok(window)
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
