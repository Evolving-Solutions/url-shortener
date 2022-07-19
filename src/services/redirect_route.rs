use actix_web::{HttpResponseBuilder, Responder, http::header::LOCATION};
use crate db::connectdb::{ConnectDB};
// Redirect incoming requests with a short_url to the long_url endpoint.
fn redirect_route (short_url: String) -> impl Responder {
    // connect to the database
    
    // match short_url to an entry in the database.
    pub const LOCATION: HeaderName;
    // if it exists, redirect to the long_url.
    LOCATION = long_url;
    // if it doesn't exist, return a 404 error.
    
}