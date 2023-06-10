use actix_web::{delete, get, HttpResponse, post, Responder, web};
use log::error;
use serde::{Deserialize, Serialize};

use crate::key_value::GenericDurableKeyValueStore;

#[get("/dache/{key}")]
pub async fn get_entry(
    (key, store): (web::Path<String>, web::Data<Box<dyn GenericDurableKeyValueStore>>),
) -> impl Responder {
    store
        .get(&key.into_inner())
        .await
        .map(|maybe_entry| match maybe_entry {
            Some(entry) => HttpResponse::Ok().body(serde_json::to_string(&entry).unwrap()),
            None => HttpResponse::NotFound().finish(),
        })
        .unwrap_or_else(|error| {
            error!("{error}");
            HttpResponse::InternalServerError().finish()
        })
}

#[derive(Serialize, Deserialize)]
pub struct SetEntryPayload {
    value: String,
}

#[post("/dache/{key}")]
pub async fn set_entry(
    (key, entry, store): (
        web::Path<String>,
        web::Json<SetEntryPayload>,
        web::Data<Box<dyn GenericDurableKeyValueStore>>,
    ),
) -> impl Responder {
    store
        .set(&key.into_inner(), &entry.value)
        .await
        .map(|_| HttpResponse::NoContent().finish())
        .unwrap_or_else(|error| {
            error!("{error}");
            HttpResponse::InternalServerError().finish()
        })
}

#[delete("/dache/{key}")]
pub async fn remove_entry(
    (key, store): (web::Path<String>, web::Data<Box<dyn GenericDurableKeyValueStore>>),
) -> impl Responder {
    store
        .remove(&key.into_inner())
        .await
        .map(|_| HttpResponse::NoContent().finish())
        .unwrap_or_else(|error| {
            error!("{error}");
            HttpResponse::InternalServerError().finish()
        })
}
