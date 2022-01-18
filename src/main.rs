use std::io;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use youtube_dl::YoutubeDl;

async fn process_urls(urls: &Vec<String>) -> String {
    for url in urls {
        let result = YoutubeDl::new(url).socket_timeout("15").run().unwrap();
        match result {
            youtube_dl::YoutubeDlOutput::Playlist(_playlist) => {return (*_playlist).entries.unwrap()[0].description.to_string() },
            youtube_dl::YoutubeDlOutput::SingleVideo(_single_video) => {return (*_single_video).description.unwrap().to_string()}
        }
    }
    return "".to_string()
}

async fn enact_directive(directive: &Directive){
    println!("{},{},{}", directive.stream_index, directive.start_sample, directive.start_insert)
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("{\"version\": \"v1.0.0\"}")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[derive(Deserialize)]
#[derive(Debug)]
struct Directive {
    stream_index: u32,
    start_sample: u32,
    start_insert: u32,
}

#[derive(Deserialize)]
struct TrackAssemblyInfo {
    urls: Vec<String>,
    directives: Vec<Directive>
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