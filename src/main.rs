use std::{collections::HashMap, sync::Mutex};

use actix_web::{
    http::header,
    web::{self},
    App, HttpServer, Responder,
};
use rand::Rng;

struct InMemoryDB {
    url_kv: std::sync::Mutex<HashMap<String, String>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(InMemoryDB {
                url_kv: Mutex::new(HashMap::new()),
            }))
            .route("/", web::get().to(index))
            .route("/s", web::get().to(shorten))
            .route("/r/{id}", web::get().to(redirect))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(serde::Deserialize)]
struct UrlQuery {
    url: String,
}
async fn shorten(info: web::Query<UrlQuery>, data: web::Data<InMemoryDB>) -> String {
    // create random 4 character string
    let random_key: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(4)
        .map(char::from)
        .collect::<String>();

    // Clone the key because we need to use it also in the response
    data.url_kv
        .lock()
        .unwrap()
        .insert(random_key.clone(), info.url.clone());

    let result = format!("http://localhost:8080/r/{}", random_key);
    result
}

async fn redirect(id: web::Path<String>, data: web::Data<InMemoryDB>) -> impl Responder {
    match data.url_kv.lock().unwrap().get(id.as_str()) {
        Some(url) => actix_web::HttpResponse::TemporaryRedirect()
            .append_header((header::LOCATION, url.clone()))
            .finish(),
        None => actix_web::HttpResponse::NotFound().body("Not found"),
    }
}

async fn index() -> &'static str {
    "Hello URL Shortener!"
}
