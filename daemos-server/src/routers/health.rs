use actix_web::{HttpResponse, get, web};

#[get("")]
async fn internal_health() -> HttpResponse {
    HttpResponse::Ok().body("API is healthy")
}

#[get("/ping")]
async fn ping_pong() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(ping_pong)
        .service(web::scope("/health").service(internal_health));
}
