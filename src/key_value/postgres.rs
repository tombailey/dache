use std::boxed::Box;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::{Client, NoTls, SimpleQueryMessage, SimpleQueryRow};

use crate::key_value::{AllEntriesKeyValueStore, Creation, DurableKeyValueStore, Entry, GenericDurableKeyValueStore, ImmutableKeyValueStore, InitializableKeyValueStore, KeyValueStore, KeyValueStoreError, MutableKeyValueStore};

pub struct PostgresKeyValueStore {
    client: Arc<Client>,
    table_name: String,
}

impl KeyValueStore for PostgresKeyValueStore {}

impl DurableKeyValueStore for PostgresKeyValueStore {}

#[async_trait]
impl ImmutableKeyValueStore for PostgresKeyValueStore {
    async fn get(&self, key: &str) -> Result<Option<Entry>, KeyValueStoreError> {
        let table = &self.table_name;
        self.client
            .query(
                &format!("SELECT key, value FROM {table} WHERE key=$1;"),
                &[&key],
            )
            .await
            .map_err(|database_error| KeyValueStoreError::StoreError {
                error: Box::new(database_error),
            })
            .map(|rows| {
                rows.first().map(|row| {
                    let key: String = row.get("key");
                    let value: String = row.get("value");
                    Entry { key, value }
                })
            })
    }
}

#[async_trait]
impl MutableKeyValueStore for PostgresKeyValueStore {
    async fn set(&self, key: &str, value: &str) -> Result<(), KeyValueStoreError> {
        let table = &self.table_name;
        self.client
            .execute(
                &format!(
                    "
                        INSERT INTO {table} (key, value, expiry) VALUES ($1, $2, null)
                        ON CONFLICT (key) DO UPDATE SET value=$2, expiry=null;
                    "
                ),
                &[&key, &value],
            )
            .await
            .map_err(|database_error| KeyValueStoreError::StoreError {
                error: Box::new(database_error),
            })
            .map(|_| ())
    }

    async fn remove(&self, key: &str) -> Result<(), KeyValueStoreError> {
        let table = &self.table_name;
        self.client
            .execute(
                &format!("DELETE FROM {table} WHERE key=$1;"),
                &[&key],
            )
            .await
            .map_err(|database_error| KeyValueStoreError::StoreError {
                error: Box::new(database_error),
            })
            .map(|_| ())
    }
}

#[async_trait]
impl AllEntriesKeyValueStore for PostgresKeyValueStore {
    async fn get_all_entries(&self) -> Result<HashMap<String, Entry>, KeyValueStoreError> {
        let table = &self.table_name;
        self.client
            .simple_query(&format!("SELECT key, value FROM {table};"))
            .await
            .map_err(|database_error| KeyValueStoreError::StoreError {
                error: Box::new(database_error),
            })
            .map(|messages| {
                let rows: Vec<&SimpleQueryRow> = messages.iter()
                    .filter_map(|message| {
                        if let SimpleQueryMessage::Row(row) = message {
                            Some(row)
                        } else {
                            None
                        }
                    })
                    .collect();

                let mut key_to_entry = HashMap::with_capacity(rows.len());
                for row in rows {
                    if let (Some(key), Some(value)) = (row.get("key"), row.get("value")) {
                        key_to_entry.insert(
                            key.to_owned(),
                            Entry {
                                key: key.to_owned(),
                                value: value.to_owned(),
                            },
                        );
                    }
                }
                key_to_entry
            })
    }

    async fn remove_all_entries(&self) -> Result<(), KeyValueStoreError> {
        let table = &self.table_name;
        self.client
            .simple_query(&format!("DELETE * FROM {table};"))
            .await
            .map_err(|database_error| KeyValueStoreError::StoreError {
                error: Box::new(database_error),
            })
            .map(|_| ())
    }
}

#[async_trait]
impl InitializableKeyValueStore for PostgresKeyValueStore {
    async fn initialize(&self) -> Result<(), KeyValueStoreError> {
        let table = &self.table_name;
        self.client
            .simple_query(
                &format!(
                    "
                        CREATE TABLE IF NOT EXISTS {table} (
                        key text PRIMARY KEY,
                        value text,
                        expiry timestamp default null
                        );
                    "
                )
            )
            .await
            .map_err(|database_error| KeyValueStoreError::StoreError {
                error: Box::new(database_error),
            })
            .map(|_| ())
    }
}

fn require_env_var(name: &str) -> Result<String, KeyValueStoreError> {
    env::var(name)
        .map_err(|_| {
            KeyValueStoreError::invalid_configuration(
                format!("Missing required environment variable {name}.")
            )
        })
}

#[async_trait]
impl Creation for PostgresKeyValueStore {
    async fn create() -> Result<Box<dyn GenericDurableKeyValueStore>, KeyValueStoreError> {
        let host = require_env_var("POSTGRES_HOST")?;
        let port = require_env_var("POSTGRES_PORT")?
            .parse::<u16>()
            .map_err(|_| {
                KeyValueStoreError::invalid_configuration(
                    format!("Invalid environment variable POSTGRES_PORT.")
                )
            })?;
        let user = require_env_var("POSTGRES_USER")?;
        let password = require_env_var("POSTGRES_PASSWORD")?;
        let database = require_env_var("POSTGRES_DATABASE")?;

        let connection_string = format!("postgres://{user}:{password}@{host}:{port}/{database}");

        let (client, connection) = tokio_postgres::connect(
            &connection_string,
            NoTls,
        )
            .await?;

        tokio::spawn(async move {
            connection.await.expect("Postgres connection error");
        });

        Ok(
            Box::new(PostgresKeyValueStore::create_with(client))
        )
    }
}

trait ParameterizedCreation {
    fn create_with(client: Client) -> PostgresKeyValueStore;
}

impl ParameterizedCreation for PostgresKeyValueStore {
    fn create_with(client: Client) -> PostgresKeyValueStore {
        PostgresKeyValueStore {
            client: Arc::new(client),
            // TODO: allow custom table name
            table_name: "dache".to_owned(),
        }
    }
}

impl From<tokio_postgres::error::Error> for KeyValueStoreError {
    fn from(error: tokio_postgres::error::Error) -> KeyValueStoreError {
        KeyValueStoreError::StoreError {
            error: Box::new(error),
        }
    }
}
