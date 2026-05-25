use rocket::{
    catch, catchers, fs::{FileServer, relative}, get, launch, routes
};
use rocket_dyn_templates::{Template};

pub mod api;
pub mod blog;
pub mod inner_voices;

pub mod software;

#[launch]
fn rocket() -> _ {
    let mut rocket_app = rocket::build()
    .mount("/public", FileServer::from(relative!("public")))
        .register("/", catchers![not_found])
        .mount("/", routes![index])
        .attach(Template::fairing());

        rocket_app = api::mount_routes(rocket_app);
    rocket_app = software::mount_routes(rocket_app);
    rocket_app = inner_voices::mount_routes(rocket_app);
    rocket_app = blog::mount_routes(rocket_app);

    rocket_app
}

#[get("/")]
fn index() -> &'static str {
    "Hello There"
}

#[catch(404)]
fn not_found() -> &'static str {
    "Page not found."
}

// fn render_markdown_simple(
//     path: PathBuf,
// ) -> Result<String, rocket::response::status::NotFound<String>> {
//     // Retrive the raw text.
//     let raw_markdown = std::fs::read_to_string(path.as_path())
//         .map_err(|_| rocket::response::status::NotFound("Not found.".into()))?;

//     // Parse it indo markdown.
//     let arena = comrak::Arena::new();
//     let options = &comrak::Options {
//         extension: comrak::ExtensionOptions::builder().table(true).build(),
//         parse: comrak::ParseOptions::builder().build(),
//         render: comrak::RenderOptions::builder().unsafe_(true).build(),
//     };
//     let markdown_tree = comrak::parse_document(&arena, raw_markdown.as_str(), options);

//     // Render the Markdown into HTML.
//     let mut raw_html = vec![];
//     comrak::format_html(markdown_tree, options, &mut raw_html)
//         .expect("Error whilst formatting HTML.");
//     let raw_html = String::from_utf8(raw_html).expect("Error parsing comrak HTML as UTF-8.");

//     Ok(raw_html)
// }
