use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, Result};
use std::path::PathBuf;

async fn index() -> Result<NamedFile> {
    let path: PathBuf = "./frontend/static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(fs::Files::new("/", "./frontend/static"))
    })
    .bind("127.0.0.1:8086")?
    .run()
    .await
}
