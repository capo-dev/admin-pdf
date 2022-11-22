use actix_files as fs;
use actix_web::{App, HttpServer};

mod csv_court_case;
mod error;
mod harris;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(harris::harris)
            .default_service(fs::Files::new("/", "./dist/").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
