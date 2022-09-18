use actix_web::{delete, get, HttpResponse, post, Responder, web};
use log::error;

use crate::key_value::{Entry, ImmutableKeyValueStore, MutableKeyValueStore};
use crate::MemoryKeyValueStore;

#[get("/dache/{key}")]
pub async fn get_entry(
    (
        key,
        store
    ): (
        web::Path<String>,
        web::Data<MemoryKeyValueStore>
    )
) -> impl Responder {
    store.get(&key.to_string())
        .await
        .map(|maybe_entry| {
            match maybe_entry {
                Some(entry) => HttpResponse::Ok()
                    .body(
                        serde_json::to_string(&entry)
                            .unwrap()
                    ),
                None => HttpResponse::NotFound().finish()
            }
        })
        .unwrap_or_else(|error| {
            error!("{error}");
            HttpResponse::InternalServerError().finish()
        })
}

#[post("/dache/{key}")]
pub async fn set_entry(
    (
        key,
        entry,
        store
    ): (
        web::Path<String>,
        web::Json<Entry>,
        web::Data<MemoryKeyValueStore>
    )
) -> impl Responder {
    store.set(key.into_inner(), entry.value.clone())
        .await
        .map(|_| {
            HttpResponse::NoContent().finish()
        })
        .unwrap_or_else(|error| {
            error!("{error}");
            HttpResponse::InternalServerError().finish()
        })
}

#[delete("/dache/{key}")]
pub async fn remove_entry(
    (
        key,
        store
    ): (
        web::Path<String>,
        web::Data<MemoryKeyValueStore>
    )
) -> impl Responder {
    store.remove(&key.into_inner())
        .await
        .map(|_| {
            HttpResponse::NoContent().finish()
        })
        .unwrap_or_else(|error| {
            error!("{error}");
            HttpResponse::InternalServerError().finish()
        })
}
