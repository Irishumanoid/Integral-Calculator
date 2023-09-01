extern crate actix_web;
extern crate actix_files;
extern crate console;

use actix_files as fs;
use actix_web::{App, HttpServer, web, middleware, Result, Responder};

use std::path::PathBuf;

use console::Style;

fn single_page_app() -> Result<fs::NamedFile> {
    // 1.
    let path: PathBuf = PathBuf::from("frontend/public/index.html");
    Ok(fs::NamedFile::open(path)?)
}

pub fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let blue = Style::new()
        .blue();
    
    // 2.
    let prefix = "0.0.0.0:"; // // Use 0.0.0.0 instead of localhost or 127.0.0.1 to use Actix with docker
    let port = 8000; // We will use 80 for aws with env variable.
    let target = format!("{}{}", prefix, port);

    println!("\nServer ready at {}", blue.apply_to(format!("http://{}",&target)));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            // 3.
            .route("/", web::get().to(single_page_app))
            .route("/user", web::get().to(single_page_app))
            .service(fs::Files::new("/", "/public").index_file("index.html"))
    })
    .bind(&target) // Separate prefix, port, target, println! not to show "Not registered service error"
    .unwrap()
    .run()
    .unwrap();
}