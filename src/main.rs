use std::path::Path;

use api::{api_catchers, api_routes};
use rocket::{
    catch, catchers,
    fs::{relative, FileServer},
    get, launch,
    response::status::NotFound,
    routes,
};
use rocket_dyn_templates::{context, Template};

pub mod api;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/public", FileServer::from(relative!("public")))
        .mount("/", routes![index, blog])
        .register("/", catchers![not_found])
        .mount("/api", api_routes())
        .register("/api", api_catchers())
        .attach(Template::fairing())
}

#[get("/")]
fn index() -> Template {
    Template::render("base", context! {})
}

#[get("/blog/<article>")]
fn blog(article: &str) -> Result<Template, NotFound<String>> {
    let markdown = std::fs::read_to_string(Path::new(format!("./blog/{}.md", article).as_str()))
        .map_err(|_| NotFound(not_found()))?;
    let content = comrak::markdown_to_html(markdown.as_str(), &comrak::Options::default());

    Ok(Template::render(
        "blog",
        context! { title: "UI Composer".to_string(), content: content },
    ))
}

#[catch(404)]
fn not_found() -> String {
    "Not found.".into()
}
