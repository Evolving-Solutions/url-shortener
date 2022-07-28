use actix_files::{Files, NamedFile};
use actix_web::{middleware::Logger, App, HttpRequest, HttpResponse, HttpServer, Responder};
use env_logger;
use handlebars::Handlebars;
use paperclip::actix::{
    api_v2_operation,
    // If you prefer the macro syntax for defining routes, import the paperclip macros
    // get, post, put, delete
    // use this instead of actix_web::web
    web::{self, Json, Route},
    Apiv2Schema,
    // extension trait for actix_web::App and proc-macro attributes
    OpenApiExt,
};
use serde_json::json;
use std::env;
mod db;
mod functions;
mod routes;
mod services;

use routes::url;

/// Greet function to test
/// # Name: greet
/**
 * Description:
 * This function will greet the user with a message.
 * It will also return the name of the user.
 * # Arguments:
 * name: The name of the user.
 * # Returns:
 *
 */
async fn greet(req: HttpRequest) -> impl Responder {
    // Declare a variable to hold a name, if no name add World to the name variable.
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

/// # Index Data
/// ## Description: Handles the data loading for handlebars on the main index page.
/// ## Privacy: Private
async fn index_data(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    // Define the data to be used on the index page,
    // This will eventually read directly from the API
    let data = json!({
        "name": "Evolving Software",
    });
    //     define the body of the handler and render the data to the template
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

/// # Main web server
/// Serves as the main entry point to the application.
/// Publicly accessible.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    env::set_var("RUST_BACKTRACE", "full");
    // Declare handlebars as a engine
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/Website_Templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    // // List the names of the database in that deployment
    // for db_name in client.list_database_names(None, None).await {
    //         println!("{:#?}", db_name);
    //     }
    HttpServer::new(|| {
        App::new()
            .wrap_api()
            //     .wrap(Logger::default())
            //     .wrap(Logger::new("%a %{User-Agent}i"))
            //     // Clone the handlebars ref to the application to give access to the engine.
            //     .app_data(handlebars_ref.clone())
            //     // Register the templates to be used that are stored in the static directory.
            //     // .service(Files::new("/static", "static").show_files_listing())
            //     // .route("/", web::get().to(index_data))
            //     // .service(url::get_url)
            //     // .service(url::create_url)
            //     // .service(url::redirect_route)
            .route(
                "/web/{url}",
                web::post().to(routes::url::redirect_old_route),
            )
            //     // .route("/greet", web::get().to(greet))
            //     // .route("/hello/{name}", web::get().to(greet))
            .with_json_spec_at("/api/spec/v2")
            .build()
    })
    .bind("0.0.0.0:8844")?
    .run()
    .await
}
