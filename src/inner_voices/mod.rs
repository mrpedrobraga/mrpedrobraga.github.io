use rocket::{get, routes, Build, Rocket};
use rocket_dyn_templates::{context, Template};

pub fn mount_routes(ro: Rocket<Build>) -> Rocket<Build> {
    ro.mount("/inner_voices", routes![base])
}

#[get("/")]
fn base() -> Template {
    Template::render("inner_voices_home", context! {})
}
