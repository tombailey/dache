use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

pub use error::KeyValueStoreError;
pub use memory::{Creation, MemoryKeyValueStore};

// TODO: durable store
mod memory;
mod error;

#[async_trait]
pub trait ImmutableKeyValueStore {
    async fn get(&self, key: &str) -> Result<Option<Entry>, KeyValueStoreError>;
}

#[async_trait]
pub trait MutableKeyValueStore {
    async fn set(&self, key: String, value: String) -> Result<(), KeyValueStoreError>;
    async fn remove(&self, key: &str) -> Result<(), KeyValueStoreError>;
}

#[async_trait]
pub trait KeyValueStore: ImmutableKeyValueStore + MutableKeyValueStore {}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    // TODO: include expiry
    pub value: String,
}

#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum DurabilityEngine {
    Memory
}

impl DurabilityEngine {
    pub fn from_str(value: &str) -> Option<DurabilityEngine> {
        [
            DurabilityEngine::Memory
        ]
            .iter()
            .find(|engine| value.to_lowercase() == engine.to_string().to_lowercase())
            .cloned()
    }
}
