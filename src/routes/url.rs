pub use crate::db::models::url::Url;
pub use crate::functions::generate::*;
use actix_web::{delete, get, http::header, post, web, Error, HttpResponse};
use chrono::prelude::*;
use local_ip_address::local_ip;
use mongodb::{
    bson,
    bson::{doc, Document},
    Client, Collection, Cursor, IndexModel,
};
// use paperclip::actix::{
//     api_v2_operation,
//     // If you prefer the macro syntax for defining routes, import the paperclip macros
//     // get, post, put, delete
//     // use this instead of actix_web::web
//     web::{self, Json},
//     Apiv2Schema,
//     HttpResponseWrapper,
//     // extension trait for actix_web::App and proc-macro attributes
//     OpenApiExt,
// };
use serde::{Deserialize, Serialize};
use std::env;

/// # Name: get_all_urls
/// Decsription: Gets all the urls in the collection.

// #[get("/url/getAll")]
// pub async fn get_all_urls() -> HttpResponse {
//
// create a mongo connection url that uses the local ip address
// let local_ip = local_ip().unwrap();
// let mongo_prefix = "mongodb+srv://";
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
pub async fn get_url(client: web::Data<Client>, search: web::Path<String>) -> HttpResponse {
    // create a mongo connection url that uses the local ip address
    let local_ip = local_ip().unwrap();
    let mongo_prefix = "mongodb+srv://";
    let mongo_prefix_and_ip = mongo_prefix + &local_ip.to_string();
    let mongo_uri = mongo_prefix_and_ip + ":27017";
    let mongo_connection_string = mongo_uri + "/evolving_solutions?retryWrites=true&w=majority";
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| mongo_connection_string.into());

    // Specify the database name
    let client = Client::with_uri_str(uri).await.expect("failed to connect");

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

/// URL Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct FormData {
    long_url: String,
    // a url_code may be present in the request body, but it is not required.
    url_code: Option<String>,
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
pub async fn create_url(form: web::Form<FormData>) -> HttpResponse {
    let local_ip = local_ip().unwrap();
    let mongo_prefix = "mongodb+srv://";
    let mongo_prefix_and_ip = mongo_prefix + &local_ip.to_string();
    let mongo_uri = mongo_prefix_and_ip + ":27017";
    let mongo_connection_string = mongo_uri + "/evolving_solutions?retryWrites=true&w=majority";
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| mongo_connection_string.into());

    // Specify the database name
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    /***
     * Determine is the request contains a long_url code.
     * If it does, then we need to check if the code is valid.
     * If it is valid, then we need to check if the code is already in the database.
     * If it is, then we need to return the url.
     * If it is not, then we need generate a new code, then check if it is in the database.
     * If it is, then we need to return an error, and generate a new code.
     * If it is not, then we need to insert the url into the database.
     */

    // Todo: Check if the short_url is already in the database.

    // Declare a mutable variable to hold the url_code from the request or a generated code.
    // check if form.url_code is = Some("");
    // if it is, then we need to generate a code.
    // if it is not, then we need to check if the code is in the database.
    // if it is, then we need to return the url.

    let url_code = form.url_code.clone().unwrap_or_else(|| generate_url_code());

    // Create a struct to hold the data and model it with the URL struct. Assign the data to the struct.
    // This will hold tangible data soon.
    let url = doc! {
        "long_url": form.long_url.clone(),
        "short_url": std::env::var("BASE_URL").unwrap_or_else( |_|"http://localhost:8080".into()) + "/" + &url_code,
        "url_code": url_code.clone(),
        "shorten_date": Utc::now().to_string(),
    };

    println!("This is the URL Code: {}", url_code.clone());
    // check if database collection evolving_solutions exists if it does not then create it.

    // Get the collection
    let collection = client
        .database("evolving_solutions")
        .collection("url_shortner");

    // Insert the data into the collection
    collection
        .insert_one(url, None)
        .await
        .expect("Failed to insert document");

    // find the collection that was just created using the Long URL.
    // Return the data back in the response body to the client with JSON.
    // Use unique index to make sure the URL is unique.
    let filter = doc! {"long_url": form.long_url.clone()};
    let url = collection
        .find_one(Some(filter), None)
        .await
        .expect("Failed to find URL");
    match url {
        Some(url) => {
            let url: Url = bson::from_bson(bson::Bson::Document(url)).unwrap();
            HttpResponse::Ok().json(url)
        }
        None => HttpResponse::NotFound().body("URL not found"),
    }
}

/// Delete a URL from the database
/// #[api_v2_operation]
#[delete("/{url_code}")]
pub async fn delete_url(url_code: web::Path<String>) -> HttpResponse {
    // connect to the database
    let local_ip = local_ip().unwrap();
    let mongo_prefix = "mongodb+srv://";
    let mongo_prefix_and_ip = mongo_prefix + &local_ip.to_string();
    let mongo_uri = mongo_prefix_and_ip + ":27017";
    let mongo_connection_string = mongo_uri + "/evolving_solutions?retryWrites=true&w=majority";
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| mongo_connection_string.into());

    let search_param = url_code.into_inner();
    let filter = doc! {"url_code": search_param };
    // Specify the database name
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
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

//Get URL by Short URL
// Mark operations like so...
// #[api_v2_operation]
#[get("/{url_code}")]
pub async fn redirect_route(url_code: web::Path<String>) -> HttpResponse {
    // connect to the database
    let local_ip = local_ip().unwrap();
    let mongo_prefix = "mongodb+srv://";
    let mongo_prefix_and_ip = mongo_prefix + &local_ip.to_string();
    let mongo_uri = mongo_prefix_and_ip + ":27017";
    let mongo_connection_string = mongo_uri + "/evolving_solutions?retryWrites=true&w=majority";
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| mongo_connection_string.into());
    // Specify the database name
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    println!("Creating client");
    // refrence the relevant collections
    let collection = client
        .database("evolving_solutions")
        .collection("url-shortener");
    println!("Creating collection");
    let search_param = url_code.into_inner();
    println!("Creating search param");
    let filter = doc! {"url_code": search_param };
    println!("Creating search filter");
    let long_url = collection
        .find_one(Some(filter), None)
        .await
        .ok()
        .expect("Error looking for url.");
    println!("Retrieved URL");
    // return the long_url
    println!("Matching URL");
    let response = HttpResponse::NotFound().body("URL not found");
    match long_url {
        Some(url) => {
            let url: Url = bson::from_bson(bson::Bson::Document(url)).unwrap();
            HttpResponse::Found()
                .append_header(("LOCATION", url.long_url))
                .finish()
        }
        None => response,
    }
}
