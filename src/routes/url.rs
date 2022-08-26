use std::any::Any;

use crate::db;
pub use crate::db::models::url::{CreateUrl, Url};
pub use crate::functions::generate::*;
use actix_web::{delete, get, post, web, HttpResponse};
use chrono::prelude::*;
use db::connectdb::database::connect_db;
use local_ip_address::local_ip;
use mongodb::{
    bson,
    bson::{doc, Document},
    Collection,
};
use serde::{Deserialize, Serialize};

/// URL Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct FormData {
    long_url: String,
    // a url_code may be present in the request body, but it is not required.
    url_code: Option<String>,
}

/// # Name: get_all_urls
/// Decsription: Gets all the urls in the collection.

// #[get("/url/getAll")]
// pub async fn get_all_urls() -> HttpResponse {
//
// create a mongo connection url that uses the local ip address
// let local_ip = local_ip().unwrap();
// let mongo_prefix = "mongodb://";
// let mongo_prefix_and_ip = mongo_prefix + &local_ip.to_string();
// let mongo_uri = mongo_prefix_and_ip + ":27017";
// let mongo_connection_string = mongo_uri + "/evolving_solutions?retryWrites=true&w=majority";
// let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| mongo_connection_string.into());

//     let client = Client::with_uri_str(uri).await.expect("Failed to connect.");
//
//     // Specify the collection name
//     let collection = client.database("evolving_solutions").collection("url-shortener");
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
/// Description: Get a url by refrences
// #[api_v2_operation]
#[get("/url/?{search}")]
pub async fn get_url(search: web::Path<String>) -> HttpResponse {
    // create a mongo connection url that uses the local ip address
    let client = connect_db().await;

    // Specify the collection name
    let collection = client.database("url_shortner").collection("url-shortener");

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
///
// #[api_v2_operation]
#[post("/url")]
pub async fn create_url(form: web::Form<CreateUrl>) -> HttpResponse {
    // copy the form into a new struct
    // check if the url_code is present in the request body, if not generate a new one.
    let mut url = form.into_inner();
    if url.url_code.is_empty() {
        url.url_code = generate_url_code()
    }

    let local_ip = local_ip().unwrap();
    let port = ":8844".to_string();
    let server_ip = local_ip.clone().to_string();
    let server_w_port = server_ip + &port;
    let server_uri = "http://".to_owned() + &server_w_port;

    let new_url = Url {
        long_url: url.long_url,
        url_code: url.url_code.clone(),
        short_url: std::env::var("BASE_URL").unwrap_or(server_uri) + "/" + &url.url_code,
        shorten_date: bson::DateTime::from_chrono(Utc::now()),
    };

    println!("This is the URL Code: {}", url.url_code.clone());
    // check if database collection evolving_solutions exists if it does not then create it.
    let client = connect_db().await;
    // Get the collection
    let collection: Collection<Url> = client
        .database("evolving_solutions")
        .collection("url_shortener");

    // Insert the data into the collection
    collection
        .insert_one(new_url.clone(), None)
        .await
        .expect("Failed to insert document");

    // find the collection that was just created using the Long URL.
    // Return the data back in the response body to the client with JSON.
    // Use unique index to make sure the URL is unique.
    let filter = doc! {"long_url": new_url.long_url.clone()};
    match collection.find_one(filter, None).await {
        Ok(Some(url)) => HttpResponse::Ok().json(url),
        Ok(None) => HttpResponse::NotFound().body("URL not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}

/// Delete a URL from the database
/// #[api_v2_operation]
#[delete("/{url_code}")]
pub async fn delete_url(url_code: web::Path<String>) -> HttpResponse {
    let client = connect_db().await;
    let search_param = url_code.into_inner();
    let filter = doc! {"url_code": search_param };
    // Specify the database name
    // create a mongo connection url that uses the local ip address
    let collection: Collection<Document> = client
        .database("evolving_solutions")
        .collection("url_shortner");

    let result = collection
        .delete_one(filter, None)
        .await
        .expect("Error deleting URL");
    println!("Deleted URL");

    match result.deleted_count {
        1 => HttpResponse::Ok().body("URL deleted"),
        _ => HttpResponse::NotFound().body("URL not found"),
    }
}

#[derive(Deserialize, Debug)]
pub struct UrlCode {
    url_code: String,
}

//Get URL by Short URL
// Mark operations like so...
// #[api_v2_operation]
#[get("/{url_code}")]
pub async fn redirect_route(url_code_path: web::Path<UrlCode>) -> HttpResponse {
    let client = connect_db().await;
    let collection: Collection<Url> = client
        .database("evolving_solutions")
        .collection("url_shortener");
    let filter = doc! {"url_code": url_code_path.url_code.clone()};
    match collection.find_one(filter, None).await {
        Ok(Some(url)) => HttpResponse::Found()
            .append_header(("LOCATION", url.long_url.clone()))
            .finish(),
        Ok(None) => HttpResponse::NotFound().body("URL not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}

#[get("/api/{url_code}")]
pub async fn redirect_v1_api_route(url_code_path: web::Path<UrlCode>) -> HttpResponse {
    println!("{:?}", url_code_path);
    let client = connect_db().await;
    let collection: Collection<Url> = client
        .database("evolving_solutions")
        .collection("url_shortener");
    let filter = doc! {"url_code": url_code_path.url_code.clone()};
    match collection.find_one(filter, None).await {
        Ok(Some(url)) => HttpResponse::Found()
            .append_header(("LOCATION", url.long_url.clone()))
            .finish(),
        Ok(None) => HttpResponse::NotFound().body("URL not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}
