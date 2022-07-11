use actix_web::{HttpRequest, HttpResponse, Responder};

#[get("health_check")]
pub async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
