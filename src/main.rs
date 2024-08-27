mod db;
mod handler;
mod model;
mod route;
mod config;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::error::Error;
use std::io::Read;
use std::net::SocketAddr;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base_port = 8000;
    let max_retries = 10;

    for port in base_port..(base_port + max_retries) {
        let addr = SocketAddr::new("0.0.0.0".parse()?, port);
        match HttpServer::new(|| {
            App::new()
                .service(hello)
                .route("/hey", web::get().to(manual_hello))
        })
            .bind(addr) {
            Ok(server) => {
                println!("Started server on {}", addr);
                server.run().await?;
            }
            Err(_) => {
                continue;
            }
        }
    }
    Ok(())
}