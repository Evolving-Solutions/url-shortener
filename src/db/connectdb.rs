pub mod database {
    use local_ip_address::local_ip;
    use mongodb::Client;

    pub async fn connect_db() -> Client {
        let local_ip = local_ip().unwrap();
        let mongo_prefix = "mongodb://";
        let mongo_prefix_and_ip = mongo_prefix.to_owned() + &local_ip.to_string();
        let mongo_uri = mongo_prefix_and_ip.to_owned() + ":27017";
        let mongo_connection_string =
            mongo_uri.to_owned() + "/evolving_solutions?retryWrites=true&w=majority";
        let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| mongo_connection_string.into());
        // create a mongo connection url that uses the local ip address
        let client = Client::with_uri_str(uri).await.expect("failed to connect");
        client
    }
}
