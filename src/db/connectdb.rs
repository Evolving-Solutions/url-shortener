pub mod database {
    use local_ip_address::local_ip;
    use mongodb::Client;

    pub async fn connect_db() -> Client {
        let local_ip = local_ip().unwrap();
        let server_ip = local_ip.clone().to_string();
        let port = ":8844".to_string();
        let mongo_uri = "mongodb://".to_string()
            + &server_ip
            + &port
            + "/evolving_solutions?retryWrites=true&w=majority";
        let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| mongo_uri.into());
        // create a mongo connection url that uses the local ip address
        let client = Client::with_uri_str(uri).await.expect("failed to connect");
        client
    }
}
