use actix_web::{App, HttpServer};
mod controllers;
mod db;
mod routes;
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
// use db::connectdb;
use routes::echo::echo;
use routes::hello::hello;
use routes::url;

/// # Main web server
/// Serves as the main entry point to the application.
/// Publicly accessible.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Parse the connection string into and options struct
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    // Specify the database name
    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    // List the names of the database in that deployment
    for db_name in client.list_database_names(None, None).await {
        println!("{:#?}", db_name);
    }
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            // Get URL's
            .service(url::get_url)
            .service(url::create_url)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
