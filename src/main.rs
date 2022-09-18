#![feature(trait_alias)]

use std::env;

use actix_web::{App, HttpServer, web};

use crate::key_value::{Creation, DurabilityEngine, GenericKeyValueStore, KeyValueStore, MemoryKeyValueStore};
use crate::router::{get_entry, get_health, remove_entry, set_entry};

mod router;
mod key_value;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .expect("Missing port.")
        .parse::<u16>()
        .expect("Invalid port.");

    DurabilityEngine::from_str(
        &env::var("DURABILITY_ENGINE")
            .expect("Missing durability engine.")
    )
        .expect("Invalid durability engine.");

    let store: Box<GenericKeyValueStore> = Box::new(MemoryKeyValueStore::create());
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
