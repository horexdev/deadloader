//! Checks on the Tauri configuration that keep the webview off the network (ADR 0001): the CSP, where the UI comes
//! from, and what the UI may call. They read the files the build uses, so loosening one fails a test.

use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

const CONFIG: &str = include_str!("../tauri.conf.json");
const MANIFEST: &str = include_str!("../Cargo.toml");

/// Our in-process protocols the UI may reach, as `https://<name>.localhost` (WebView2 has no custom schemes, so
/// Tauri maps each one to such an origin and serves it without touching the network).
const OWN_PROTOCOLS: &[&str] = &["ipc"];

/// Permissions the UI may hold. Empty: no command is callable yet; never `shell:`, `fs:` or `http:`.
const ALLOWED_PERMISSIONS: &[&str] = &[];

fn config() -> Value {
    serde_json::from_str(CONFIG).expect("tauri.conf.json is JSON")
}

/// Directive name → its sources.
fn parse_csp(csp: &str) -> BTreeMap<String, Vec<String>> {
    csp.split(';')
        .map(str::trim)
        .filter(|d| !d.is_empty())
        .map(|d| {
            let mut parts = d.split_whitespace();
            let name = parts.next().unwrap_or_default().to_owned();
            (name, parts.map(str::to_owned).collect())
        })
        .collect()
}

fn csp() -> BTreeMap<String, Vec<String>> {
    let config = config();
    let csp = config["app"]["security"]["csp"]
        .as_str()
        .expect("the CSP is set as one string");
    parse_csp(csp)
}

fn is_own_protocol(source: &str) -> bool {
    source
        .strip_prefix("https://")
        .and_then(|host| host.strip_suffix(".localhost"))
        .is_some_and(|name| OWN_PROTOCOLS.contains(&name))
}

#[test]
fn csp_parser_splits_directives_and_sources() {
    let parsed = parse_csp("default-src 'self'; connect-src a b ;");
    assert_eq!(parsed["default-src"], ["'self'"]);
    assert_eq!(parsed["connect-src"], ["a", "b"]);
    assert_eq!(parsed.len(), 2);
}

#[test]
fn csp_defaults_to_self_only() {
    assert_eq!(csp()["default-src"], ["'self'"]);
}

#[test]
fn csp_allows_no_remote_origin_and_nothing_unsafe() {
    for (directive, sources) in csp() {
        for source in sources {
            let allowed = source == "'self'" || source == "'none'" || is_own_protocol(&source);
            assert!(allowed, "{directive} allows {source}");
        }
    }
}

#[test]
fn csp_blocks_plugins_base_and_form_targets() {
    let csp = csp();
    for directive in ["object-src", "base-uri", "form-action", "frame-ancestors"] {
        assert_eq!(csp[directive], ["'none'"], "{directive}");
    }
}

#[test]
fn own_protocol_check_rejects_lookalikes() {
    assert!(is_own_protocol("https://ipc.localhost"));
    assert!(!is_own_protocol("http://ipc.localhost"));
    assert!(!is_own_protocol("https://ipc.localhost.example.com"));
    assert!(!is_own_protocol("https://evil.localhost"));
    assert!(!is_own_protocol("https://localhost"));
}

#[test]
fn the_ui_is_embedded_and_never_loaded_from_a_server() {
    let config = config();
    let build = &config["build"];
    assert!(build.get("devUrl").is_none(), "{build}");
    let dist = build["frontendDist"]
        .as_str()
        .expect("frontendDist is a path");
    assert!(!dist.contains("://"), "{dist}");
    // tauri's `custom-protocol` feature turns off its dev mode, which would load `devUrl`.
    let tauri = MANIFEST
        .lines()
        .find(|l| l.starts_with("tauri = "))
        .expect("tauri dependency");
    assert!(tauri.contains("\"custom-protocol\""), "{tauri}");
}

#[test]
fn no_tray_no_devtools_no_asset_protocol() {
    let tauri = MANIFEST
        .lines()
        .find(|l| l.starts_with("tauri = "))
        .expect("tauri dependency");
    assert!(tauri.contains("default-features = false"), "{tauri}");
    for feature in ["tray-icon", "devtools", "protocol-asset", "isolation"] {
        assert!(!tauri.contains(feature), "{feature} in {tauri}");
    }
}

#[test]
fn windows_are_created_in_code_with_our_webview_settings() {
    let config = config();
    let windows = config["app"]["windows"].as_array().expect("windows list");
    assert!(windows.is_empty(), "{windows:?}");
    assert_eq!(config["app"]["withGlobalTauri"], Value::Bool(false));
}

#[test]
fn the_ui_holds_only_allowed_permissions() {
    let config = config();
    if let Some(inline) = config["app"]["security"].get("capabilities") {
        assert!(
            inline.as_array().is_some_and(Vec::is_empty),
            "capabilities belong in files: {inline}"
        );
    }
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("capabilities");
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return;
    };
    for entry in entries {
        let path = entry.expect("entry").path();
        let text = std::fs::read_to_string(&path).expect("capability file");
        let capability: Value = serde_json::from_str(&text).expect("capability JSON");
        for permission in capability["permissions"].as_array().into_iter().flatten() {
            let id = permission
                .as_str()
                .or_else(|| permission["identifier"].as_str())
                .unwrap_or_default();
            assert!(
                ALLOWED_PERMISSIONS.contains(&id),
                "{} grants {id}",
                path.display()
            );
        }
    }
}
