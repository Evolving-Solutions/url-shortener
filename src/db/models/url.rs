use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Url {
    pub id: u32,
    pub longUrl: String,
    pub shortUrl: String,
    pub urlCode: String 
}