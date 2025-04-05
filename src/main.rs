use api::{api_catchers, api_routes};

use rocket::{
    catch, catchers,
    fs::{relative, FileServer},
    get, launch,
    routes,
};
use rocket_dyn_templates::{context, Template};

pub mod api;
pub mod inner_voices;
pub mod blog;

#[launch]
fn rocket() -> _ {
    let mut rocket_app = rocket::build()
        // Mounting the home!
        .mount("/", routes![index])
        .register("/", catchers![not_found])
        .mount("/public", FileServer::from(relative!("public")))

        // Mounting the API!
        .mount("/api", api_routes())
        .register("/api", api_catchers())

        // Attach the template fairing
        .attach(Template::fairing());

    // Mounting the Inner Voices subpage.
    rocket_app = inner_voices::mount_routes(rocket_app);

    // Mounting the blog subpage.
    rocket_app = blog::mount_routes(rocket_app);

    rocket_app
}

#[get("/")]
fn index() -> Template {
    Template::render("base", context! {})
}

#[catch(404)]
fn not_found() -> String {
    "Not found.".into()
}
