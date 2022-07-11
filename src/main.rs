use actix_web::{App, HttpServer};
mod controllers;
mod db;
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
// use db::connectdb;
use crate::routes::*;

/// # Main web server
/// Serves as the main entry point to the application.
/// Publicly accessible.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // // List the names of the database in that deployment
    // for db_name in client.list_database_names(None, None).await {
    //     println!("{:#?}", db_name);
    // }
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(health_check)
            .service(greet)
            // Get URL's
            .service(url::get_url)
            .service(url::create_url)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
