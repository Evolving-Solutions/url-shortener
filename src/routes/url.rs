use actix_web::{get, post, HttpResponse, Responder};
use mongodb::{bson::doc, Client, Collection, IndexModel};

/// # Name: URL Getter
/// Description: Get urls
#[get("/url")]
pub async fn get_url() -> impl Responder {
    HttpResponse::Ok().body("URL's go here")
}
/// # Path /url
///
/// Request Type: POST
///
/// Description: main route to create a url
///
/// Parameters:
///
/// Name: req_body
/// Type: String
///
/// Name: client
/// Type: Client
#[post("/url")]
pub async fn create_url(req_body: String) -> impl Responder {
    HttpResponse::Ok()
        .body(req_body)
}
