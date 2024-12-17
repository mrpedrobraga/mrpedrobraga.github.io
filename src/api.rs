use rocket::{catch, catchers, get, routes, Request, Responder};

pub fn api_routes() -> Vec<rocket::Route> {
    routes![root]
}

pub fn api_catchers() -> Vec<rocket::Catcher> {
    catchers![not_found]
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct RawJson(pub &'static str);

#[get("/")]
pub fn root() -> RawJson {
    RawJson("{\"name\": \"Pedro Braga\", \"age\": 21}")
}

#[derive(Responder)]
#[response(status = 404, content_type = "json")]
pub struct NotFoundError(());

#[catch(404)]
pub fn not_found(_req: &Request) -> Result<(), NotFoundError> {
    return Err(NotFoundError(()));
}
