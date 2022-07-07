use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use std::num::ParseIntError;

struct ConnectDB {
    URI: u32,
    client: Client,
}

impl ConnectDB {
    // Parse the connection string into and options struct
    async fn connectdb(&self) {
        let uri =
            std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

        // Specify the database name
        let client = Client::with_uri_str(uri).await.expect("failed to connect");
    }
}
