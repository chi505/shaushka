mod lib;
pub use lib::*;

use std::io;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("{\"version\": \"v1.0.0\"}")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/payload")]
async fn payload(payload: web::Json<TrackAssemblyInfo>) -> impl Responder {
    let urls_string = process_urls(&payload.urls).await;
    for directive in &payload.directives {
        enact_directive(directive).await
    }
    HttpResponse::Ok().body(format!("{}",urls_string))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(healthcheck)
            .service(echo)
            .service(payload)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}