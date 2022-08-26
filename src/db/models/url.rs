use mongodb::bson::DateTime;
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

/// # Name: URL Struct
/// Description: Struct to hold the data for the URL's
///
/// # Fields
///
/// Name: id
/// Type: u32
/// Description: Unique ID for the URL
///
/// Name: longUrl
/// Type: Char
/// Description: Long URL
///
/// Name: shortUrl
/// Type: Char
/// Description: Short URL
///
/// Name: urlCode
/// Type: Char
/// Description: Unique code for the URL ( Can be manually set or generated )
///
/// # Methods
///
/// Name: new
/// Description: Constructor for the URL struct
///
/// Name: set_id
/// Description: Set the ID for the URL
///
/// Name: set_longUrl
/// Description: Set the Long URL for the URL
///
/// Name: set_shortUrl
/// Description: Set the Short URL for the URL
///

// A url struct that is able to be cloned
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Url {
    pub long_url: String,
    pub short_url: String,
    pub url_code: String,
    pub shorten_date: DateTime,
}
#[derive(Deserialize, Serialize, Debug, Clone, Apiv2Schema)]
pub struct CreateUrl {
    pub long_url: String,
    pub url_code: String,
}
