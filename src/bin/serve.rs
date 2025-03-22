use actix_files::Files;
use actix_web::{ App, HttpServer };
use std::path::PathBuf;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let web_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("web");
    println!("Serving files from: {}", web_dir.display());
    println!("Visit http://127.0.0.1:8080 to see the web app");

    HttpServer::new(move || {
        App::new().service(Files::new("/", &web_dir).index_file("index.html"))
    })
        .bind("127.0.0.1:8080")?
        .run().await
}
