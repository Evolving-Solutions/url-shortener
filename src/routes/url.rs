use actix_web::{get, post, web, HttpResponse, Responder, Result};
use chrono::prelude::*;
use mongodb::{bson, bson::doc, Client, Collection, IndexModel};
use serde::Deserialize;

pub use crate::db::models::url::Url;

/// # Name: URL Getter
/// Description: Get urls
#[get("/url/{search}")]
pub async fn get_url(client: web::Data<Client>, search: web::Path<String>) -> HttpResponse {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    // Specify the database name
    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    // Specify the collection name
    let collection = client.database("url-shortener").collection("urls");

    let searchParam = search.into_inner();

    // Create a filter to search for the URL
    let filter = doc! {"longUrl": searchParam};

    // Find the URL
    let url = collection
        .find_one(Some(filter), None)
        .await
        .expect("Failed to find URL");

    // Return the URL
    match url {
        Some(url) => {
            let url: Url = bson::from_bson(bson::Bson::Document(url)).unwrap();
            HttpResponse::Ok().json(url)
        }
        None => HttpResponse::NotFound().body("URL not found"),
    }
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
pub async fn create_url(form: web::Form<FormData>) -> HttpResponse {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    // Specify the database name
    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    // Create a struct to hold the data and model it with the URL struct. Assign the data to the struct.
    let url = Url {
        longUrl: form.longUrl.clone(),
        shortUrl: "".to_string(),
        urlCode: "".to_string(),
        shortenDate: Utc::now().to_string(),
    };

    // Get the collection
    let collection: Collection<Url> = client.database("url-shortener").collection("urls");

    // Insert the data into the collection
    collection
        .insert_one(url, None)
        .await
        .expect("Failed to insert document");

    // find the collection that was just created using the Long URL and return all the data
    match collection
        .find_one(doc! {"longUrl": form.longUrl.clone()}, None)
        .await
    {
        Ok(result) => match result {
            Some(result) => {
                let url = result.clone();
                HttpResponse::Ok().json(url)
            }
            None => HttpResponse::Ok().json("Unable to shorten URL, please try again."),
        },
        Err(e) => HttpResponse::Ok().json(e.to_string()),
    }
}
