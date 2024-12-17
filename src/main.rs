use api::{api_catchers, api_routes};
use rocket::{
    catch, catchers,
    fs::{relative, FileServer},
    get, launch, routes,
};
use rocket_dyn_templates::{context, Template};

pub mod api;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/public", FileServer::from(relative!("public")))
        .mount("/", routes![index])
        .register("/", catchers![not_found])
        .mount("/api", api_routes())
        .register("/api", api_catchers())
        .attach(Template::fairing())
}

#[get("/")]
fn index() -> Template {
    Template::render("base", context! {})
}

#[catch(404)]
fn not_found() -> String {
    "Not found.".into()
}
