use lazy_static::lazy_static;
use dioxus::prelude::*;
use bevy_pkv::PkvStore;
use serde::{Serialize, de::DeserializeOwned};
use std::sync::Mutex;

lazy_static! {
    static ref PKV_STORE: Mutex<PkvStore> = Mutex::new(PkvStore::new("dev-widgets", "persistence"));
}

pub fn use_persistent<T: Serialize + DeserializeOwned + Default + 'static>(
    cx: &ScopeState,
    // A unique key for the storage entry
    key: impl ToString,
    // A function that returns the initial value if the storage entry is empty
    init: impl FnOnce() -> T,
) -> &UsePersistent<T> {
    // Use the use_ref hook to create a mutable state for the storage entry
    let state = use_ref(cx, move || {
        // This closure will run when the hook is created
        let key = key.to_string();
        let value = PKV_STORE.lock().unwrap().get(key.clone()).ok().unwrap_or_else(init);
        StorageEntry { key, value }
    });

    // Wrap the state in a new struct with a custom API
    // Note: We use use_hook here so that this hook is easier to use in closures in the rsx. Any values with the same lifetime as the ScopeState can be used in the closure without cloning.
    cx.use_hook(|| UsePersistent {
        inner: state.clone(),
    })
}

struct StorageEntry<T> {
    key: String,
    value: T,
}

/// Storage that persists across application reloads
pub struct UsePersistent<T: 'static> {
    inner: UseRef<StorageEntry<T>>,
}

impl<T: Serialize + DeserializeOwned + Clone + 'static> UsePersistent<T> {
    /// Returns a reference to the value
    pub fn get(&self) -> T {
        self.inner.read().value.clone()
    }

    /// Sets the value
    pub fn set(&self, value: T) {
        let mut inner = self.inner.write();
        // Write the new value to local storage
        PKV_STORE.lock().unwrap().set(inner.key.clone(), &value).unwrap();
        inner.value = value;
    }
}