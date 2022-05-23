// /// Type: Module
// /// Privacy: Public
// /// Description: MongoDB connection to the database
// /// Dependencies: None
// /// Version: 1.0.0
// /// Language: rust
// /// Path: server\src\db\connectdb.rs
// ///
// /// # Module
// /// Module for connecting to the database.
// ///
// /// ## Functions
// ///
// /// ### connect
// /// Connect to the database.
// ///
// /// ## Variables
// ///
// /// ### db
// /// Database connection.
// use mongodb::{options::ClientOptions, Client};

// pub struct ConnectDb {
//     pub db: Client
// }



// /// Title: ConnectDB
// /// Description: Connect the database
// pub async fn connectDB() -> ArrayVec<Client> {
//     // Parse the connection string into and options struct
//     let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

//     // Specify the database name
//     client_options.app_name = Some("url-shortener".to_string());

//     // Get a handle to the client
//     let client = Client::with_options(client_options);

//     // List the names of the database in that deployment
//     for db_name in client.list_database_names(None, None).await {
//         println!("{}", db_name);
//     }
// }
