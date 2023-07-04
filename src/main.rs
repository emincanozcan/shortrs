use std::{collections::HashMap, sync::Mutex};

use actix_web::{
    http::{
        header::{self, ContentType},
        StatusCode,
    },
    web::{self},
    App, HttpResponse, HttpServer, Responder,
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
            .route("/", web::get().to(list))
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

async fn list(data: web::Data<InMemoryDB>) -> impl Responder {
    let mut result = String::new();
    result.push_str("<link rel='stylesheet' href='https://cdn.jsdelivr.net/npm/purecss@3.0.0/build/pure-min.css' integrity='sha384-X38yfunGUhNzHpBaEBsWLO+A0HDYOQi8ufWDkZ0k9e0eXz/tH3II7uKZ9msv++Ls' crossorigin='anonymous'>");
    result.push_str("<div style='margin: 2em auto; width: 40em; max-width: 100%;'>");
    result.push_str("<h1>List of all URLs</h1>");
    result.push_str("<table class='pure-table' style='width: 100%;'>");
    result.push_str("<thead><tr><th>Key</th><th>URL</th><th>Go to</th></tr><thead>");
    result.push_str("<tbody>");
    for (key, value) in data.url_kv.lock().unwrap().iter() {
        result.push_str("<tr>");
        result.push_str("<td>");
        result.push_str(key);
        result.push_str("</td>");
        result.push_str("<td>");
        result.push_str(value);
        result.push_str("</td>");
        result.push_str("<td>");
        result.push_str(&format!(
            "<a target='blank' href='http://localhost:8080/r/{}'>Click here</a>",
            key
        ));
        result.push_str("</td>");
        result.push_str("</tr>");
    }
    result.push_str("</tbody>");
    result.push_str("</table>");
    result.push_str("</div>");
    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(result)
}
