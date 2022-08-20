use actix_files::{Files, NamedFile};
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use env_logger;
use routes::url;
use serde_json::json;
use std::env;
// use paperclip::actix::{
//     api_v2_operation,
//     // If you prefer the macro syntax for defining routes, import the paperclip macros
//     // get, post, put, delete
//     // use this instead of actix_web::web
//     web::{self, Json, Route},
//     Apiv2Schema,
//     // extension trait for actix_web::App and proc-macro attributes
//     OpenApiExt,
// };
mod db;
mod functions;
mod routes;
mod services;

/// # Main web server
/// Serves as the main entry point to the application.
/// Publicly accessible.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    env::set_var("RUST_BACKTRACE", "full");
    HttpServer::new(|| {
        App::new()
            // .route("/web/{url}", web::get().to(routes::url::redirect_old_route))
            .service(url::get_url)
            .service(url::create_url)
            .service(url::redirect_route)
            .service(url::delete_url)
        // .with_json_spec_at("/api/spec/v2")
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
