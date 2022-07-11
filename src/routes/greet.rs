use actix_web::{get, HttpResponse, HttpRequest, Responder};

pub async fn greet(req: HttpRequest) -> impl Responder  {
     //Define a variable to hold the name from the request if no name, add world.
    let name :&str = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}