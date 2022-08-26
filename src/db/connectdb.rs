pub mod database {
    extern crate dotenv;
    use dotenv::dotenv;
    use local_ip_address::local_ip;
    use mongodb::{options::ClientOptions, Client};
    use std::env;

    pub async fn connect_db() -> Client {
        dotenv().ok();
        let local_ip = local_ip().unwrap();
        let mongo_prefix = "mongodb://";
        let mongo_prefix_and_ip = mongo_prefix.to_owned() + &local_ip.to_string();
        let mongo_uri = mongo_prefix_and_ip.to_owned() + ":27017";
        let mongo_connection_string =
            mongo_uri.to_owned() + "/evolving_solutions?directConnection=true&retryWrites=true";
        let uri = env::var("MONGODB_URI").unwrap_or(mongo_connection_string);
        println!("Connecting to Mongodb with connection string: {:?}", uri);
        let client_connection = uri.to_string();
        let client_options = ClientOptions::parse(client_connection);
        match client_options {
            Ok(options) => {
                let mut client_options = options;
                client_options.app_name = Some("url-shortener".to_string());
                let client = Client::with_options(client_options);
                match client {
                    Ok(client) => client,
                    Err(e) => {
                        println!("Failed to connect: {}", e);
                        panic!("Failed to connect: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
                panic!("Failed to connect.");
            }
        }
    }
}
