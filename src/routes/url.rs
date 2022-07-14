pub use crate::db::models::url::Url;
pub use crate::functions::generate::*;
use actix_web::{
    get, post,
    web::{self, Form},
    HttpResponse, Responder, Result,
};
use chrono::prelude::*;
use mongodb::{
    bson,
    bson::{doc, Document},
    Client, Collection, Cursor, IndexModel,
};
use serde::Deserialize;
use std::env;

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
//     let collection = client.database("project-k").collection("url-shortener");
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
#[get("/url/{search}")]
pub async fn get_url(client: web::Data<Client>, search: web::Path<String>) -> HttpResponse {
    let uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb+srv://admin:spike2@project-k-dev-api.evolvingsoftware.io/project-k/?retryWrites=true&w=majority".into());

    // Specify the database name
    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    // Specify the collection name
    let collection = client.database("project-k").collection("url-shortener");

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
#[derive(Deserialize, Debug)]
struct FormData {
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
#[post("/url")]
async fn create_url(form: web::Form<FormData>) -> HttpResponse {
    let uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://admin:admin@127.0.0.1/?retryWrites=true&w=majority".into());

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

    let url_code;
    if form.url_code.clone().unwrap() == "" {
        url_code = generate_url_code();
    } else {
        url_code = form.url_code.clone().unwrap();
    };

    // println!("{}", generate_url_code());
    // println!(
    //     "THis is the generated code: {}",
    //     generated_code.clone().to_string()
    // );
    // Print the form data to the console.

    // fn get_url_code(form: <FormData>) -> String {
    //     let mut temp_url_code = form.url_code.clone();
    //  temp_url_code.get_or_insert(generate_url_code()).to_string()
    // }

    // Check if the url_code is present in the request body.
    // if it is not present, then we need to generate a code.
    // if temp_url_code.is_empty() {
    //     url_code = generate_url_code().to_string();
    // } else {
    //     url_code = form.url_code.clone().to_string();
    // };

    // Create a struct to hold the data and model it with the URL struct. Assign the data to the struct.
    // This will hold tangible data soon.
    let url = doc! {
        "long_url": form.long_url.clone(),
        "short_url": std::env::var("BASE_URL").unwrap_or_else( |_|"http://localhost:8000".into()) + "/" + &url_code,
        "url_code": url_code.clone(),
        "shorten_date": Utc::now().to_string(),
    };

    println!("This is the URL Code: {}", url_code.clone());
    // Get the collection
    let collection = client.database("project-k").collection("url-shortener");

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


