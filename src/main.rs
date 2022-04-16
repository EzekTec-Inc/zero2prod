//! main.rs
#![allow(dead_code)]
#![allow(unused_variables)]

/* # use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
 async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}*/
use rust_zero2prod::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to port");
    run(listener)?.await
    //  Spawn a new tcp port and use that to instantiate `run()`
    // let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port");

    // We retrieve the port here again, but this time its for `main`
    // let port = listener.local_addr().unwrap().port();

    //  Bubble up the io::Error if we failed to bind the address
    //  otherwise call .await on our HttpServer
    // let server = run(listener)?;

    // let _ = tokio::spawn(server);

    // Ok(())

    /* HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await */
}
