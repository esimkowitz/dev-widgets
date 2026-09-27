use dioxus::prelude::*;
use dioxus_sdk::storage::{use_storage, LocalStorage};
use serde::{de::DeserializeOwned, Serialize};

/// Signal backed by localStorage (web) or a file in the app data dir (desktop).
///
/// Falls back to `init` if the key is missing or no longer deserializes.
/// Keys double as filenames on desktop, so stick to `[a-z0-9.-]`.
pub fn use_local_persistent<T>(key: &str, init: impl FnOnce() -> T) -> Signal<T>
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync + PartialEq + 'static,
{
    use_storage::<LocalStorage, T>(format!("dev-widgets.{key}"), init)
}
