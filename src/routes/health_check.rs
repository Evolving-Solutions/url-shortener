use actix_web::{get, App, HttpRequest, HttpServer, HttpResponse, Responder};
use actix_web::web::to;
use crate::health_check;

// #[get("/health_check")]
// pub async fn health_check(req: HttpRequest) -> impl Responder {
//     HttpResponse::Ok()
// }

pub async fn hello() -> impl Responder {
   health_check
}