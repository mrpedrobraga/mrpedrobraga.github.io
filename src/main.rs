use api::api_routes;
use rocket::{
    fs::{relative, FileServer},
    get, launch, routes,
};
use rocket_dyn_templates::{context, Template};

pub mod api;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/public", FileServer::from(relative!("/public")))
        .mount("/", routes![index])
        .mount("/api", api_routes())
        .attach(Template::fairing())
}

#[get("/")]
fn index() -> Template {
    Template::render("base", context! { title: "mrpedrobraga" })
}
