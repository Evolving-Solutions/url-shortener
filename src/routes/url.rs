use actix_web::{get, post, HttpResponse, Responder, web, Result};
use mongodb::{bson::doc, Client, Collection, IndexModel};
use serde::Deserialize;

/// # Name: URL Getter
/// Description: Get urls
#[get("/url")]
pub async fn get_url() -> impl Responder {
    HttpResponse::Ok().body("URL's go here")
}

/// URL Struct
#[derive(Deserialize)]
struct FormData {
    longUrl: String,
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
pub async fn create_url(path: web::Path<(u32, String)>) -> Result<String> {
    println!("{}", req_body);
    // PARSE THE BODY
    HttpResponse::Ok().body("Printed response to the server console");
}
