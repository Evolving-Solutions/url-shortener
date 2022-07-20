pub use crate::db::connectdb;
use actix_web::{
    get,
    http::header::LOCATION,
    web::{self, Form},
    HttpResponse, HttpResponseBuilder, Responder,
};
use mongodb::{
    bson,
    bson::{doc, Document},
    Client, Collection,
};
// Redirect incoming requests with a short_url to the long_url endpoint.
//     // match short_url to an entry in the database.
//
//     //ToDO write failing test for this
//     pub const LOCATION: HeaderName;
//     // if it exists, redirect to the long_url.
//     LOCATION = long_url;
//     // if it doesn't exist, return a 404 error.
// }
