use std::net::TcpListener;

use actix_web::dev::Server;

pub use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health-check", web::get().to(health_check))
            .route("/newsletters", web::post().to(newsletter_subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn newsletter_subscribe(_form: web::Form<Newsletter>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct Newsletter {
    email: String,
    name: String,
}
