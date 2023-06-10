use std::collections::HashMap;

use async_trait::async_trait;
use dashmap::DashMap;

use crate::key_value::{AllEntriesKeyValueStore, Creation, DurableKeyValueStore, Entry, GenericDurableKeyValueStore, ImmutableKeyValueStore, InitializableKeyValueStore, KeyValueStore, KeyValueStoreError, MutableKeyValueStore};

pub struct MemoryKeyValueStore {
    values: DashMap<String, String>,
}


impl KeyValueStore for MemoryKeyValueStore {}

impl DurableKeyValueStore for MemoryKeyValueStore {}

#[async_trait]
impl InitializableKeyValueStore for MemoryKeyValueStore {
    async fn initialize(&self) -> Result<(), KeyValueStoreError> {
        Ok(())
    }
}

#[async_trait]
impl AllEntriesKeyValueStore for MemoryKeyValueStore {
    async fn get_all_entries(&self) -> Result<HashMap<String, Entry>, KeyValueStoreError> {
        let mut key_to_value: HashMap<String, Entry> = HashMap::new();
        self.values.iter().for_each(|entry| {
            key_to_value.insert(
                entry.key().to_owned(),
                Entry {
                    key: entry.key().to_owned(),
                    value: entry.value().to_owned(),
                },
            );
        });
        Ok(key_to_value)
    }

    async fn remove_all_entries(&self) -> Result<(), KeyValueStoreError> {
        self.values.clear();
        Ok(())
    }
}

#[async_trait]
impl ImmutableKeyValueStore for MemoryKeyValueStore {
    async fn get(&self, key: &str) -> Result<Option<Entry>, KeyValueStoreError> {
        Ok(
            self.values.get(key).map(|entry| {
                Entry {
                    key: entry.key().to_owned(),
                    value: entry.value().to_owned(),
                }
            })
        )
    }
}

#[async_trait]
impl MutableKeyValueStore for MemoryKeyValueStore {
    async fn set(&self, key: &str, value: &str) -> Result<(), KeyValueStoreError> {
        self.values.insert(key.to_owned(), value.to_owned());
        Ok(())
    }

    async fn remove(&self, key: &str) -> Result<(), KeyValueStoreError> {
        self.values.remove(key);
        Ok(())
    }
}

#[async_trait]
impl Creation for MemoryKeyValueStore {
    async fn create() -> Result<Box<dyn GenericDurableKeyValueStore>, KeyValueStoreError> {
        Ok(
            Box::new(
                Self::create_with(HashMap::new())
            )
        )
    }
}

trait ParameterizedCreation {
    fn create_with(initial: HashMap<String, String>) -> MemoryKeyValueStore;
}

impl ParameterizedCreation for MemoryKeyValueStore {
    fn create_with(initial: HashMap<String, String>) -> MemoryKeyValueStore {
        let dash_map: DashMap<String, String> = DashMap::new();
        initial.iter().for_each(|(key, value)| {
            dash_map.insert(key.to_owned(), value.to_owned());
        });
        MemoryKeyValueStore {
            values: dash_map,
        }
    }
}
