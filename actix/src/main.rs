// extern crate actix_web;
use actix_web::{HttpServer, App, Responder, web};

fn index() -> impl Responder {
    "Hello world!"
}

fn name(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

fn main() -> std::io::Result<()> {
    let server = HttpServer::new(
        || App::new()
            .route("/hello/{name}", web::get().to(name))
            .route("/", web::get().to(index))
            .route("/index.html", web::get().to(index))
    );

    println!("listening to 127.0.0.1:8088");
    server.bind("127.0.0.1:8088")?.run()
}
