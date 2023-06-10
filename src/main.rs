#![feature(trait_alias)]

use std::env;

use actix_web::{App, HttpServer, web};

use crate::key_value::{Creation, DurabilityEngine, GenericDurableKeyValueStore, KeyValueStoreError, MemoryKeyValueStore, PostgresKeyValueStore};
use crate::router::{get_entry, get_health, remove_entry, set_entry};

mod key_value;
mod router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .expect("Missing port.")
        .parse::<u16>()
        .expect("Invalid port.");

    let engine = DurabilityEngine::from_str(
        &env::var("DURABILITY_ENGINE").expect("Missing DURABILITY_ENGINE.")
    ).expect("Invalid DURABILITY_ENGINE.");

    let store = create_durable_key_value_store(engine)
        .await
        .expect("Failed to create durability engine.");
    let app_data = web::Data::new(store);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(get_entry)
            .service(set_entry)
            .service(remove_entry)
            .service(get_health)
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}

async fn create_durable_key_value_store(
    engine: DurabilityEngine
) -> Result<Box<dyn GenericDurableKeyValueStore>, KeyValueStoreError> {
    let durability_engine = match engine {
        DurabilityEngine::Memory => MemoryKeyValueStore::create().await,
        DurabilityEngine::Postgres => PostgresKeyValueStore::create().await,
    }?;

    durability_engine.initialize().await?;
    Ok(durability_engine)
}
