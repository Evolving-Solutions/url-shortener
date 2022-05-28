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
#[derive( Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Url {
    pub longUrl: String,
    pub shortUrl: String,
    pub urlCode: String,
    pub shortenDate: String,
}
