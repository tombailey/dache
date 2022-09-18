use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::key_value::{Entry, ImmutableKeyValueStore, KeyValueStoreError, MutableKeyValueStore};

pub struct MemoryKeyValueStore {
    values: Arc<RwLock<HashMap<String, String>>>,
}

#[async_trait]
impl ImmutableKeyValueStore for MemoryKeyValueStore {
    async fn get(
        &self, key: &str,
    ) -> Result<Option<Entry>, KeyValueStoreError> {
        self.values.read()
            .await
            .get(key)
            .map_or(
                Ok(None),
                |value| Ok(
                    Some(
                        Entry {
                            value: value.to_owned()
                        }
                    )
                ),
            )
    }
}

#[async_trait]
impl MutableKeyValueStore for MemoryKeyValueStore {
    async fn set(
        &self, key: String, value: String,
    ) -> Result<(), KeyValueStoreError> {
        self.values.write()
            .await
            .insert(key, value);
        Ok(())
    }

    async fn remove(
        &self, key: &str,
    ) -> Result<(), KeyValueStoreError> {
        self.values.write()
            .await
            .remove(key);
        Ok(())
    }
}

pub trait Creation {
    fn create() -> MemoryKeyValueStore;
    fn create_with(initial: HashMap<String, String>) -> MemoryKeyValueStore;
}

impl Creation for MemoryKeyValueStore {
    fn create() -> MemoryKeyValueStore {
        Self::create_with(HashMap::new())
    }

    fn create_with(initial: HashMap<String, String>) -> MemoryKeyValueStore {
        MemoryKeyValueStore {
            values: Arc::new(RwLock::new(initial))
        }
    }
}
