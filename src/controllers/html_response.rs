use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

pub async fn html_response(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("html").parse().unwrap();
    Ok(NamedFile::open(path)?)
}
