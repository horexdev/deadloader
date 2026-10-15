//! The catalog (S2, S3) and its pictures' consent (D2).

use super::*;

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
