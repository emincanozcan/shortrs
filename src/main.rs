use askama::Template;
use actix_web::{
    http::{
        header::{self, ContentType},
        StatusCode,
    },
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use rand::Rng;
mod database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(
                database::Database::new("database.json").unwrap()
            ))
            .route("/", web::get().to(list))
            .route("/shorten", web::post().to(shorten))
            .route("/r/{id}", web::get().to(redirect))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    data: Vec<(String, String)>,
}
async fn list(data: web::Data<database::Database>) -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(IndexTemplate { data: data.get_all() }.render().unwrap())
}

#[derive(serde::Deserialize)]
struct FormContent {
    url: String,
}
async fn shorten(payload: web::Form<FormContent>, data: web::Data<database::Database>) -> impl Responder {
    // create random 4 character string
    let random_key: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(4)
        .map(char::from)
        .collect::<String>();

    // Clone the key because we need to use it also in the response
    data.store_kv(random_key.clone(), payload.url.clone()).unwrap();
    // redirect to the homepage 301
    HttpResponse::MovedPermanently()
        .append_header((header::LOCATION, "/"))
        .finish()
}

async fn redirect(id: web::Path<String>, data: web::Data<database::Database>) -> impl Responder {
    match data.get_value(id.as_str()) {
        Some(url) => HttpResponse::TemporaryRedirect()
            .append_header((header::LOCATION, url.clone()))
            .finish(),
        None => HttpResponse::NotFound().body("Not found"),
    }
}
