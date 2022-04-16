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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //  Bubble up the io::Error if we failed to bind the address
    //  otherwise call .await on our HttpServer
    let _ = run("127.0.0.1:0").await?;

    Ok(())

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
