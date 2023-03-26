use actix_web::{web, App, HttpResponse, HttpServer, dev::Server};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

