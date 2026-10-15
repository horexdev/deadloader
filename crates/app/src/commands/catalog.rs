//! The catalog (S2, S3) and its pictures' consent (D2).

use super::*;
use deadloader_core::log::{DiskLogDir, Level, Logger, SystemClock};

/// A catalog page; `by_user` when the user asked for it (scrolling to it, searching), not a background load.
#[tauri::command]
pub async fn catalog_list(
    slot: Slot<'_>,
    query: Query,
    by_user: bool,
) -> Result<Listing, Failure<CatalogError>> {
    on_catalog(slot, move |c| crate::catalog::list(c, &query, by_user)).await
}

/// Pull to refresh / Retry: the page from GameBanana now.
#[tauri::command]
pub async fn catalog_refresh(
    slot: Slot<'_>,
    query: Query,
) -> Result<Listing, Failure<CatalogError>> {
    on_catalog(slot, move |c| crate::catalog::refresh(c, &query)).await
}

/// The next page at ~70 % scroll (28); `false` when nothing was fetched.
#[tauri::command]
pub async fn catalog_prefetch(slot: Slot<'_>, query: Query) -> Reply<bool> {
    on_catalog(slot, move |c| crate::catalog::prefetch(c, &query)).await
}

#[tauri::command]
pub async fn catalog_categories(
    slot: Slot<'_>,
    section: Section,
    parent: Option<u64>,
) -> Result<Categories, Failure<CatalogError>> {
    on_catalog(slot, move |c| {
        crate::catalog::categories(c, section, parent)
    })
    .await
}

/// A mod page; `listed_updated` is the update date the card showed (Unix seconds), to tell a saved page is current.
#[tauri::command]
pub async fn catalog_details(
    slot: Slot<'_>,
    section: Section,
    id: u64,
    listed_updated: Option<i64>,
) -> Result<ModPage, Failure<CatalogError>> {
    on_catalog(slot, move |c| {
        crate::catalog::details(c, section, id, listed_updated)
    })
    .await
}

/// The global network indicator's state; changes arrive as pushes afterwards.
#[tauri::command]
pub async fn catalog_status(slot: Slot<'_>) -> Reply<Status> {
    on_catalog(slot, crate::catalog::status).await
}

/// "Retry now" in E5.
#[tauri::command]
pub async fn catalog_retry_now(slot: Slot<'_>) -> Reply<()> {
    on_catalog(slot, crate::catalog::retry_now).await
}

/// A click on a description image (29).
#[tauri::command]
pub async fn catalog_activate(slot: Slot<'_>, id: String) -> Result<(), Failure<ConsentError>> {
    on_catalog(slot, move |c| crate::catalog::activate(c, &id)).await
}

/// What dialog D2 shows for an image from a host not allowed yet.
#[tauri::command]
pub async fn catalog_consent_prompt(
    slot: Slot<'_>,
    id: String,
) -> Result<ConsentPrompt, Failure<ConsentError>> {
    on_catalog(slot, move |c| crate::catalog::consent_prompt(c, &id)).await
}

/// The user's answer in D2 (122).
#[tauri::command]
pub async fn catalog_grant(
    slot: Slot<'_>,
    id: String,
    scope: Scope,
) -> Result<Granted, Failure<ConsentError>> {
    on_catalog(slot, move |c| crate::catalog::grant(c, &id, scope)).await
}

/// The catalog's favourites (34), newest first: "favourites only" lists them.
#[tauri::command]
pub async fn favourites(backend: Shared<'_>) -> Reply<Vec<deadloader_core::favourites::Favourite>> {
    blocking(backend, move |b| b.favourites()).await
}

/// The heart on a card or the mod page; the whole list comes back.
#[tauri::command]
pub async fn set_favourite(
    backend: Shared<'_>,
    favourite: deadloader_core::favourites::Favourite,
    on: bool,
) -> Reply<Vec<deadloader_core::favourites::Favourite>> {
    blocking(backend, move |b| b.set_favourite(favourite, on)).await
}

/// D3 (109): the user confirmed legal age before showing mods with content ratings. Saved and logged.
#[tauri::command]
pub async fn confirm_age(backend: Shared<'_>, log: AppLogState<'_>) -> Reply<Settings> {
    let log = Arc::clone(log.inner());
    blocking(backend, move |b| {
        let saved = b.confirm_age()?;
        deadloader_catalog::AppLog::write(
            &*log,
            Level::Info,
            "legal age confirmed for mods with content ratings",
            serde_json::Map::new(),
        );
        Ok(saved)
    })
    .await
}

/// D1's Open (31): the page goes to the user's default browser. Only https pages leave (`shell::may_open`).
#[tauri::command]
pub async fn open_link(log: AppLogState<'_>, url: String) -> Reply<()> {
    let log = Arc::clone(log.inner());
    tauri::async_runtime::spawn_blocking(move || {
        open_with(&url, deadloader_platform_windows::shell::open_url, &*log)
    })
    .await
    .map_err(|e| UserMessage::new(MessageId::Internal).arg("detail", e))?
}

type AppLogState<'a> = State<'a, Arc<Logger<SystemClock, DiskLogDir>>>;

/// Hands `url` to `open` and logs what happened, opened or not.
fn open_with(
    url: &str,
    open: impl FnOnce(&str) -> std::io::Result<()>,
    log: &dyn deadloader_catalog::AppLog,
) -> Reply<()> {
    let mut fields = serde_json::Map::from_iter([("url".to_owned(), url.into())]);
    match open(url) {
        Ok(()) => {
            log.write(Level::Info, "link opened in the browser", fields);
            Ok(())
        }
        Err(e) => {
            fields.insert("error".to_owned(), e.to_string().into());
            log.write(Level::Warn, "link not opened", fields);
            Err(UserMessage::new(MessageId::Internal)
                .arg("detail", format!("the link could not be opened: {e}")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    type Line = (Level, String, serde_json::Map<String, serde_json::Value>);

    #[derive(Default)]
    struct Lines(Mutex<Vec<Line>>);

    impl deadloader_catalog::AppLog for Lines {
        fn write(
            &self,
            level: Level,
            msg: &str,
            fields: serde_json::Map<String, serde_json::Value>,
        ) -> Option<String> {
            self.0
                .lock()
                .expect("lock")
                .push((level, msg.to_owned(), fields));
            None
        }
    }

    #[test]
    fn an_opened_link_is_logged_with_its_address() {
        let lines = Lines::default();
        let mut opened = Vec::new();
        open_with(
            "https://gamebanana.com/mods/655209",
            |url| {
                opened.push(url.to_owned());
                Ok(())
            },
            &lines,
        )
        .expect("opened");
        assert_eq!(opened, ["https://gamebanana.com/mods/655209"]);
        let lines = lines.0.into_inner().expect("lines");
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].0, Level::Info);
        assert_eq!(lines[0].2["url"], "https://gamebanana.com/mods/655209");
    }

    #[test]
    fn a_refused_link_is_an_error_and_logged_as_a_warning() {
        let lines = Lines::default();
        let err = open_with(
            "http://example.org/",
            deadloader_platform_windows::shell::open_url,
            &lines,
        )
        .expect_err("only https");
        assert_eq!(err.id, MessageId::Internal);
        let lines = lines.0.into_inner().expect("lines");
        assert_eq!(lines[0].0, Level::Warn);
        assert!(lines[0].2.contains_key("error"));
    }
}
