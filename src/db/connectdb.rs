use mongodb::{options::IndexOptions, Client, Collection, IndexModel};

pub async fn connect_db() -> Client {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    // Specify the database name
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    client
}
