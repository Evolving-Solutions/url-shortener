use actix_web::{App, HttpServer};
use local_ip_address::local_ip;
use routes::url;
use std::env;
mod db;
mod functions;
mod routes;

/// # Main web server
/// Serves as the main entry point to the application.
/// Publicly accessible.
/// Log what port the server is running on.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let local_ip = local_ip().unwrap();
    let server_ip = local_ip.clone().to_string();
    let port = ":8844".to_string();
    let server_w_port = server_ip + &port;
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    env::set_var("RUST_BACKTRACE", "full");
    println!("Listen on: {}", server_w_port);
    HttpServer::new(|| {
        App::new()
            .service(url::get_url)
            .service(url::create_url)
            .service(url::redirect_route)
            .service(url::delete_url)
        // .with_json_spec_at("/api/spec/v2")
    })
    .bind(server_w_port)?
    .run()
    .await
}
