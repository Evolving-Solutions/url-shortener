use actix_web::{get, post, web, HttpResponse, Responder, Result};
use chrono::prelude::*;
use mongodb::{bson, bson::doc, Client, Collection, Cursor, IndexModel};
use serde::Deserialize;
use std::env;

pub use crate::db::models::url::Url;

/// # Name: get_all_urls
/// Decsription: Gets all the urls in the collection.

// #[get("/url/getAll")]
// pub async fn get_all_urls() -> HttpResponse {
//     let uri =
//         std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb+srv://evolvingadmin:FdXCevnY5SkWbaHH@evolving-development.jkdlu.mongodb.net/?retryWrites=true&w=majority".into());
//
//     let client = Client::with_uri_str(uri).await.expect("Failed to connect.");
//
//     // Specify the collection name
//     let collection = client.database("url-shortener").collection("urls");
//
//     // Create a filter to search for the URL
//
//     let all_urls = collection.list_indexes(None).await.unwrap();
//     while let Some(Ok(index)) = cursor.next().await {
//         if index.keys == doc! {"_id": 1} {
//             continue;
//         } else {
//             assert_eq!(index.keys, doc! {"index": 1});
//             assert_eq!(index.clone().options.unwrap().unique, None);
//             assert_eq!(index.clone().options.unwrap().sparse.unwrap(), true);
//         }
//     }
//     // Return the URL
//     match all_urls {
//         Some(all_urls) => {
//             println("Here is the data you requested: {}", e);
//             HttpResponse::Ok().finish()
//         }
//         None => HttpResponse::NotFound().body("URL not found"),
//     }
// }

/// # Name: URL Getter
/// Description: Get a url by refrence
#[get("/url/{search}")]
pub async fn get_url(client: web::Data<Client>, search: web::Path<String>) -> HttpResponse {
    let uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb+srv://evolvingadmin:FdXCevnY5SkWbaHH@evolving-development.jkdlu.mongodb.net/?retryWrites=true&w=majority".into());

    // Specify the database name
    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    // Specify the collection name
    let collection = client.database("url-shortener").collection("urls");

    let search_param = search.into_inner();

    // Create a filter to search for the URL
    let filter = doc! {"long_url": search_param};

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
    long_url: String,
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
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb+srv://admin:spike2@127.0.0.1:27017/?retryWrites=true&w=majority".into());

    // Specify the database name
    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    // Create a struct to hold the data and model it with the URL struct. Assign the data to the struct.
    // This will hold tangible data soon.
    let url = Url {
        long_url: form.long_url.clone(),
        short_url: "".to_string(),
        url_code: "".to_string(),
        shorten_date: Utc::now().to_string(),
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
        .find_one(doc! {"long_url": form.long_url.clone()}, None)
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
