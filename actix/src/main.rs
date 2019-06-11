extern crate actix_web;
use actix_web::{server, App, HttpRequest, Responder, Path};

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn name(name: Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

fn main() {
    let server = server::new(|| {
        vec![
            App::new()
                .prefix("/hello")
                .resource("/{name}", |r| r.with(name)),
            App::new().resource("/", |r| r.f(index)),
        ]
    });

    server.bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
