use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use std::num::ParseIntError;

pub struct ConnectDB {
    uri: u32,
    client: Client,
}

impl ConnectDB {
    // Parse the connection string into and options struct
    pub async fn connect_db(&self) {
        let uri =
            std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

        // Specify the database name
        let client = Client::with_uri_str(uri).await.expect("failed to connect");
    }
}
