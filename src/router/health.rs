use actix_web::{get, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use serde::Serialize;

#[derive(Serialize)]
struct Healthy {
    status: String,
}

#[get("/health")]
pub async fn get_health() -> impl Responder {
    let healthy = Healthy {
        status: String::from("pass")
    };
    let json = serde_json::to_string(&healthy).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json)
}
