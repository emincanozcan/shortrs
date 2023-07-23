use std::{collections::HashMap, sync::Mutex, io, path::Path, fs::File,io::Write};

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
    file_path: String,
}
impl InMemoryDB {
    fn new(file_path: &str) -> io::Result<Self> {
        let db = InMemoryDB {
            url_kv: Mutex::new(HashMap::new()),
            file_path: file_path.to_string(),
        };
        if Path::new(file_path).exists() {
            let file = File::open(file_path)?;
            let data: HashMap<String, String> = serde_json::from_reader(file)?;
            *db.url_kv.lock().unwrap() = data;
         }
        Ok(db)
    }

    fn save_to_file(&self) -> io::Result<()> {
        let file = File::create(&self.file_path)?;
        let data = self.url_kv.lock().unwrap();
        let json = serde_json::to_string(&data.clone())?;
        let mut writer = io::BufWriter::new(file);
        writer.write_all(json.as_bytes())?;
        writer.flush()?;
        Ok(())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(
                InMemoryDB::new("database.json").unwrap()
            ))
            .route("/", web::get().to(list))
            .route("/shorten", web::post().to(shorten))
            .route("/r/{id}", web::get().to(redirect))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(serde::Deserialize)]
struct FormContent {
    url: String,
}
// Payload is the form data contains shorten parameter
async fn shorten(payload: web::Form<FormContent>, data: web::Data<InMemoryDB>) -> impl Responder {
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
        .insert(random_key.clone(), payload.url.clone());
    data.save_to_file().unwrap();
    // redirect to the homepage 301
    HttpResponse::MovedPermanently()
        .append_header((header::LOCATION, "/"))
        .finish()
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
    result.push_str("<h2 style='margin-top: 3em'>Shorten a new URL</h2>");
    result.push_str("<form class='pure-form' action='/shorten' method='post'>");
    result.push_str("<fieldset>");
    result.push_str("<input type='text' name='url' placeholder='URL' style='width: 100%;' />");
    result.push_str("<button type='submit' style='margin-top: 1rem;' class='pure-button pure-button-primary'>Shorten</button>");
    result.push_str("</fieldset>");
    result.push_str("</form>");
    result.push_str("</div>");
    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(result)
}
