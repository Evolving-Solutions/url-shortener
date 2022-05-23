use actix_web::{get, post, HttpResponse, Responder};

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
#[post("/url")]
pub async fn create_url(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}