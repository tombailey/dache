use actix_http::Request;
use actix_web::{App, test, web};
use actix_web::http::{Method, StatusCode};
use actix_web::test::{call_and_read_body_json, call_service, init_service};
use actix_web::web::ServiceConfig;

use crate::key_value::{Entry, GenericDurableKeyValueStore, MemoryKeyValueStore};
use crate::router::{get_entry, get_health, remove_entry, set_entry};

fn create_actix_app_configurer(store: Box<dyn GenericDurableKeyValueStore>) -> impl Fn(&mut ServiceConfig) {
    let web_data = web::Data::new(store);
    move |config: &mut ServiceConfig| {
        config
            .app_data(web_data.clone())
            .service(get_entry)
            .service(set_entry)
            .service(remove_entry)
            .service(get_health);
    }
}

fn get_value_request(key: &str) -> Request {
    test::TestRequest::default()
        .method(Method::GET)
        .uri(&format!("/dache/{key}"))
        .to_request()
}

fn set_value_request(entry: &Entry) -> Request {
    let key = entry.key.clone();
    test::TestRequest::default()
        .method(Method::POST)
        .uri(&format!("/dache/{key}"))
        .set_json(entry)
        .to_request()
}

fn delete_value_request(key: &str) -> Request {
    test::TestRequest::default()
        .method(Method::DELETE)
        .uri(&format!("/dache/{key}"))
        .to_request()
}

#[actix_rt::test]
async fn test_set_value() {
    let store = MemoryKeyValueStore::default();
    let app = init_service(
        App::new()
            .configure(
                create_actix_app_configurer(Box::new(store))
            )
    )
        .await;

    let entry = Entry {
        key: String::from("test_set_key"),
        value: String::from("test_set_value"),
    };

    assert!(
        call_service(&app, set_value_request(&entry))
            .await
            .status()
            .is_success()
    );

    assert_eq!(
        entry,
        call_and_read_body_json(&app, get_value_request(&entry.key)).await
    );
}

#[actix_rt::test]
async fn test_missing_value() {
    let store = MemoryKeyValueStore::default();
    let app = init_service(
        App::new()
            .configure(
                create_actix_app_configurer(Box::new(store))
            )
    )
        .await;

    assert_eq!(
        StatusCode::NOT_FOUND,
        call_service(&app, get_value_request("test_missing_key")).await.status()
    );
}

#[actix_rt::test]
async fn test_delete_value() {
    let store = MemoryKeyValueStore::default();
    let app = init_service(
        App::new()
            .configure(
                create_actix_app_configurer(Box::new(store))
            )
    )
        .await;

    let entry = Entry {
        key: String::from("test_delete_key"),
        value: String::from("test_delete_value"),
    };

    assert!(
        call_service(&app, set_value_request(&entry))
            .await
            .status()
            .is_success()
    );

    assert!(
        call_service(&app, delete_value_request(&entry.key))
            .await
            .status()
            .is_success()
    );

    assert_eq!(
        StatusCode::NOT_FOUND,
        call_service(&app, get_value_request(&entry.key)).await.status()
    );
}
