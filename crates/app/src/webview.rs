//! How WebView2 is started (ADR 0001): our own browser arguments, so the engine has no route to the internet and its
//! online features stay off, and what is logged about the engine at start.

use std::ffi::OsString;
use std::path::Path;

/// Command-line switches for the WebView2 browser process.
const SWITCHES: &[&str] = &[
    "--disable-background-networking",
    "--disable-component-update",
    "--no-pings",
    "--disable-domain-reliability",
    // A dead proxy: even if the CSP failed, the webview could not reach the internet. Our own protocols are served
    // in-process and never go through it. `<-loopback>` removes the implicit loopback bypass, so localhost is dead too.
    "--proxy-server=127.0.0.1:9",
    "--proxy-bypass-list=<-loopback>",
];

/// Chromium and Edge features to switch off. Chromium honours only one `--disable-features` switch, so they all go
/// into one list.
const DISABLED_FEATURES: &[&str] = &[
    // wry's defaults, which our arguments replace: the mini menu, the PDF menu and SmartScreen.
    "msWebOOUI",
    "msPdfOOUI",
    "msSmartScreenProtection",
    // Proven necessary (R05): without it the browser process signs in to the Microsoft account service through
    // OneAuth/WAM on every start.
    "msOneAuthWAM",
    "AutofillServerCommunication",
    "OptimizationHints",
    "MediaRouter",
    "DnsOverHttps",
    // Proven necessary (R05): without it DNS-over-HTTPS probes go to the user's DNS provider.
    "DnsOverHttpsUpgrade",
    "AsyncDns",
    "DnsHttpssvc",
    "UseDnsHttpsSvcb",
    "msDnsOverHttps",
    "msSecureDns",
    "EncryptedClientHello",
];

/// The full argument string passed to WebView2.
pub fn browser_args() -> String {
    let mut args = SWITCHES.join(" ");
    args.push_str(" --disable-features=");
    args.push_str(&DISABLED_FEATURES.join(","));
    args
}

/// The `WEBVIEW2_*` environment variables among `vars`, sorted by name: they can change the engine's arguments,
/// profile folder or runtime, so every one that is set is logged.
pub fn webview2_variables<I>(vars: I) -> Vec<(String, String)>
where
    I: IntoIterator<Item = (OsString, OsString)>,
{
    let mut picked: Vec<(String, String)> = vars
        .into_iter()
        .map(|(name, value)| {
            (
                name.to_string_lossy().into_owned(),
                value.to_string_lossy().into_owned(),
            )
        })
        .filter(|(name, _)| {
            name.get(..PREFIX.len())
                .is_some_and(|start| start.eq_ignore_ascii_case(PREFIX))
        })
        .collect();
    picked.sort();
    picked
}

const PREFIX: &str = "WEBVIEW2_";

/// The window's zoom (Settings → General, Ctrl + wheel / plus / minus / 0): the last percentage applied, so saving any
/// other setting doesn't zoom the page again (a zoom change lays out the whole page).
#[derive(Debug, Default)]
pub struct Zoom {
    last: std::sync::atomic::AtomicU32,
}

impl Zoom {
    /// The WebView2 scale factor for `percent`, or `None` when the window already has it.
    pub fn changed(&self, percent: u32) -> Option<f64> {
        let before = self.last.swap(percent, std::sync::atomic::Ordering::SeqCst);
        (before != percent).then(|| f64::from(percent) / 100.0)
    }
}

/// The design's smallest layout, in CSS px: the window's minimum at 100 % zoom.
pub const MIN_LAYOUT: (f64, f64) = (960.0, 600.0);

/// The window's minimum inner size in logical px at zoom `factor` (decision 192): the design's smallest layout at that
/// zoom, capped at `room` (the work area of the window's monitor, logical px), never below the plain minimum.
pub fn min_window(factor: f64, room: Option<(f64, f64)>) -> (f64, f64) {
    let axis = |base: f64, room: Option<f64>| {
        let zoomed = (base * factor).round();
        room.map_or(zoomed, |room| zoomed.min(room.floor()))
            .max(base)
    };
    (
        axis(MIN_LAYOUT.0, room.map(|r| r.0)),
        axis(MIN_LAYOUT.1, room.map(|r| r.1)),
    )
}

/// Where a window of `size` at `position` goes so it lies on `area` (a monitor's work area, position and size; all
/// physical px): moved left/up just enough, never past the area's left/top edge.
pub fn keep_inside(
    position: (i32, i32),
    size: (u32, u32),
    area: ((i32, i32), (u32, u32)),
) -> (i32, i32) {
    let axis = |at: i32, length: u32, start: i32, room: u32| {
        let end = i64::from(start) + i64::from(room) - i64::from(length);
        let placed = i64::from(at).min(end).max(i64::from(start));
        i32::try_from(placed).unwrap_or(at)
    };
    (
        axis(position.0, size.0, area.0.0, area.1.0),
        axis(position.1, size.1, area.0.1, area.1.1),
    )
}

/// Whether two Windows paths name the same folder, ignoring case, separators and a trailing separator.
pub fn same_folder(a: &Path, b: &Path) -> bool {
    normalise(a) == normalise(b)
}

fn normalise(path: &Path) -> String {
    let text = path.to_string_lossy().replace('/', r"\");
    let text = text.strip_prefix(r"\\?\").unwrap_or(&text);
    text.trim_end_matches('\\').to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn the_zoom_reaches_the_window_only_when_it_changed() {
        let zoom = Zoom::default();
        assert_eq!(
            zoom.changed(100),
            Some(1.0),
            "the first value always applies"
        );
        assert_eq!(zoom.changed(100), None, "another setting was saved");
        assert_eq!(zoom.changed(125), Some(1.25));
        assert_eq!(zoom.changed(50), Some(0.5));
    }

    #[test]
    fn the_window_minimum_holds_the_designs_smallest_layout_at_any_zoom() {
        let big = Some((2560.0, 1400.0));
        assert_eq!(min_window(1.0, big), (960.0, 600.0));
        assert_eq!(min_window(1.5, big), (1440.0, 900.0));
        assert_eq!(min_window(1.1, big), (1056.0, 660.0));
        assert_eq!(
            min_window(0.5, big),
            (960.0, 600.0),
            "never below the design's window minimum"
        );
    }

    #[test]
    fn a_grown_window_moves_back_onto_its_monitor() {
        let area = ((0, 0), (1920, 1040));
        assert_eq!(keep_inside((100, 50), (1440, 900), area), (100, 50), "fits");
        assert_eq!(keep_inside((900, 400), (1440, 900), area), (480, 140));
        // A second monitor left of the main one, negative coordinates.
        let left = ((-1366, 0), (1366, 728));
        assert_eq!(keep_inside((-400, 10), (1200, 728), left), (-1200, 0));
        assert_eq!(
            keep_inside((-2000, -50), (1200, 728), left),
            (-1366, 0),
            "never above or left of it"
        );
    }

    #[test]
    fn the_window_minimum_never_outgrows_the_monitor() {
        // 1366 × 768 laptop, work area without the taskbar.
        let laptop = Some((1366.0, 728.0));
        assert_eq!(min_window(1.25, laptop), (1200.0, 728.0));
        assert_eq!(min_window(2.0, laptop), (1366.0, 728.0));
        assert_eq!(
            min_window(1.5, Some((800.0, 500.0))),
            (960.0, 600.0),
            "a screen smaller than the minimum keeps the plain minimum"
        );
        assert_eq!(min_window(1.5, None), (1440.0, 900.0), "unknown monitor");
    }

    /// The set from ADR 0001, written out independently of the constants above.
    const REQUIRED_SWITCHES: [&str; 6] = [
        "--disable-background-networking",
        "--disable-component-update",
        "--no-pings",
        "--disable-domain-reliability",
        "--proxy-server=127.0.0.1:9",
        "--proxy-bypass-list=<-loopback>",
    ];
    const REQUIRED_FEATURES: [&str; 15] = [
        "msWebOOUI",
        "msPdfOOUI",
        "msSmartScreenProtection",
        "msOneAuthWAM",
        "AutofillServerCommunication",
        "OptimizationHints",
        "MediaRouter",
        "DnsOverHttps",
        "DnsOverHttpsUpgrade",
        "AsyncDns",
        "DnsHttpssvc",
        "UseDnsHttpsSvcb",
        "msDnsOverHttps",
        "msSecureDns",
        "EncryptedClientHello",
    ];

    fn disabled_features(args: &str) -> Vec<&str> {
        let lists: Vec<&str> = args
            .split_whitespace()
            .filter_map(|a| a.strip_prefix("--disable-features="))
            .collect();
        assert_eq!(lists.len(), 1, "exactly one --disable-features in {args}");
        lists[0].split(',').collect()
    }

    #[test]
    fn every_required_switch_is_passed() {
        let args = browser_args();
        let given: BTreeSet<&str> = args.split_whitespace().collect();
        for switch in REQUIRED_SWITCHES {
            assert!(given.contains(switch), "{switch} missing from {args}");
        }
    }

    #[test]
    fn every_required_feature_is_disabled_in_a_single_list() {
        let args = browser_args();
        let given = disabled_features(&args);
        for feature in REQUIRED_FEATURES {
            assert!(given.contains(&feature), "{feature} missing from {args}");
        }
        let unique: BTreeSet<&str> = given.iter().copied().collect();
        assert_eq!(unique.len(), given.len(), "duplicates in {args}");
    }

    #[test]
    fn wry_defaults_are_kept_because_our_arguments_replace_them() {
        let args = browser_args();
        let given = disabled_features(&args);
        for feature in ["msWebOOUI", "msPdfOOUI", "msSmartScreenProtection"] {
            assert!(given.contains(&feature), "{feature} missing from {args}");
        }
    }

    #[test]
    fn nothing_is_enabled_or_left_empty() {
        let args = browser_args();
        assert!(!args.contains("--enable-features"), "{args}");
        assert!(!args.contains("--remote-debugging"), "{args}");
        assert!(
            args.split_whitespace().all(|a| a.starts_with("--")),
            "{args}"
        );
        assert!(
            disabled_features(&args).iter().all(|f| !f.is_empty()),
            "{args}"
        );
    }

    fn vars(pairs: &[(&str, &str)]) -> Vec<(OsString, OsString)> {
        pairs
            .iter()
            .map(|(k, v)| (OsString::from(k), OsString::from(v)))
            .collect()
    }

    #[test]
    fn only_webview2_variables_are_picked_sorted_by_name() {
        let picked = webview2_variables(vars(&[
            ("PATH", r"C:\Windows"),
            ("WEBVIEW2_USER_DATA_FOLDER", r"C:\elsewhere"),
            (
                "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
                "--enable-features=X",
            ),
            ("WEBVIEW2X", "not ours"),
        ]));
        assert_eq!(
            picked,
            vec![
                (
                    "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS".to_owned(),
                    "--enable-features=X".to_owned()
                ),
                (
                    "WEBVIEW2_USER_DATA_FOLDER".to_owned(),
                    r"C:\elsewhere".to_owned()
                ),
            ]
        );
    }

    #[test]
    fn variable_names_match_regardless_of_case_as_on_windows() {
        let picked = webview2_variables(vars(&[("WebView2_Release_Channel_Preference", "1")]));
        assert_eq!(picked.len(), 1);
    }

    #[test]
    fn no_variables_means_an_empty_list() {
        assert!(webview2_variables(vars(&[("TEMP", r"C:\Temp")])).is_empty());
    }

    #[test]
    fn folders_compare_like_windows_does() {
        assert!(same_folder(
            Path::new(r"D:\Games\deadloader\deadloader-data\webview"),
            Path::new(r"d:\games\DEADLOADER\deadloader-data\webview\")
        ));
        assert!(same_folder(
            Path::new(r"D:\Games\deadloader\deadloader-data\webview"),
            Path::new("D:/Games/deadloader/deadloader-data/webview")
        ));
        assert!(same_folder(
            Path::new(r"\\?\D:\Games\deadloader\deadloader-data\webview"),
            Path::new(r"D:\Games\deadloader\deadloader-data\webview")
        ));
        assert!(!same_folder(
            Path::new(r"D:\Games\deadloader\deadloader-data\webview"),
            Path::new(r"C:\Users\x\AppData\Local\io.github.horexdev.deadloader\EBWebView")
        ));
        assert!(!same_folder(
            Path::new(r"D:\Games\deadloader\deadloader-data\webview"),
            Path::new(r"D:\Games\deadloader\deadloader-data\webview2")
        ));
    }
}
