use rocket::{get, routes, Responder};

pub fn api_routes() -> Vec<rocket::Route> {
    routes![root]
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct RawJson(pub &'static str);

#[get("/")]
pub fn root() -> RawJson {
    RawJson("{\"name\": \"Pedro Braga\", \"age\": 21}")
}
