use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

pub use error::KeyValueStoreError;
pub use memory::MemoryKeyValueStore;
pub use postgres::PostgresKeyValueStore;

mod error;
mod memory;
mod postgres;

#[async_trait]
pub trait ImmutableKeyValueStore {
    async fn get(&self, key: &str) -> Result<Option<Entry>, KeyValueStoreError>;
}

#[async_trait]
pub trait MutableKeyValueStore {
    async fn set(&self, key: &str, value: &str) -> Result<(), KeyValueStoreError>;
    async fn remove(&self, key: &str) -> Result<(), KeyValueStoreError>;
}

#[async_trait]
pub trait KeyValueStore: ImmutableKeyValueStore + MutableKeyValueStore {}

pub trait GenericKeyValueStore = KeyValueStore;

#[async_trait]
pub trait AllEntriesKeyValueStore {
    async fn get_all_entries(&self) -> Result<HashMap<String, Entry>, KeyValueStoreError>;
    async fn remove_all_entries(&self) -> Result<(), KeyValueStoreError>;
}

#[async_trait]
pub trait InitializableKeyValueStore {
    async fn initialize(&self) -> Result<(), KeyValueStoreError>;
}

#[async_trait]
pub trait DurableKeyValueStore: InitializableKeyValueStore + AllEntriesKeyValueStore + GenericKeyValueStore
{}

pub trait GenericDurableKeyValueStore = DurableKeyValueStore + Sync + Send;

#[derive(Serialize, Deserialize)]
pub struct Entry {
    // TODO: include expiry
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum DurabilityEngine {
    Memory,
    Postgres,
}

impl DurabilityEngine {
    pub fn from_str(value: &str) -> Option<DurabilityEngine> {
        [
            DurabilityEngine::Memory,
            DurabilityEngine::Postgres,
        ]
            .iter()
            .find(|engine| value.to_lowercase() == engine.to_string().to_lowercase())
            .cloned()
    }
}

#[async_trait]
pub trait Creation {
    async fn create() -> Result<Box<dyn GenericDurableKeyValueStore>, KeyValueStoreError>;
}
