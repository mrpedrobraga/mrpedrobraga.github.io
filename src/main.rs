use api::{api_catchers, api_routes};
use rocket::{
    catch, catchers,
    fs::{relative, FileServer},
    get, launch, routes,
};
use rocket_dyn_templates::{context, Template};
use std::path::PathBuf;

pub mod api;
pub mod blog;
pub mod inner_voices;

pub mod software;

#[launch]
fn rocket() -> _ {
    let mut rocket_app = rocket::build()
        // Mounting the home!
        .mount("/", routes![index, videogames, art])
        .register("/", catchers![not_found])
        .mount("/public", FileServer::from(relative!("public")))
        // Mounting the API!
        .mount("/api", api_routes())
        .register("/api", api_catchers())
        // Attach the template fairing
        .attach(Template::fairing());

    rocket_app = software::mount_routes(rocket_app);

    // Mounting the Inner Voices subpage.
    rocket_app = inner_voices::mount_routes(rocket_app);

    // Mounting the blog subpage.
    rocket_app = blog::mount_routes(rocket_app);

    rocket_app
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "base",
        context! {
            title: "Home",
            gimmick_path: "~",
            path: "/",
            nav_index: 1,
            content: render_markdown_simple(PathBuf::from("./pages/home.md")).expect("Failed to get file etc")
        },
    )
}

#[get("/videogames")]
fn videogames() -> Template {
    Template::render(
        "base",
        context! {
            title: "Games",
            gimmick_path: "~/videogames",
            path: "/videogames",
            nav_index: 3,
            content: render_markdown_simple(PathBuf::from("./pages/games.md")).expect("Failed to get file etc")
        },
    )
}

#[get("/art")]
fn art() -> Template {
    Template::render(
        "base",
        context! {
            title: "Art Portfolio",
            gimmick_path: "~/art",
            path: "/art",
            nav_index: 4,
            content: render_markdown_simple(PathBuf::from("./pages/art.md")).expect("Failed to get file etc")
        },
    )
}

#[catch(404)]
fn not_found(request: &rocket::Request) -> String {
    format!("bash: {}: No such file or directory.", request.uri())
}

fn render_markdown_simple(
    path: PathBuf,
) -> Result<String, rocket::response::status::NotFound<String>> {
    // Retrive the raw text.
    let raw_markdown = std::fs::read_to_string(path.as_path())
        .map_err(|_| rocket::response::status::NotFound("Not found.".into()))?;

    // Parse it indo markdown.
    let arena = comrak::Arena::new();
    let options = &comrak::Options {
        extension: comrak::ExtensionOptions::builder().table(true).build(),
        parse: comrak::ParseOptions::builder().build(),
        render: comrak::RenderOptions::builder().unsafe_(true).build(),
    };
    let markdown_tree = comrak::parse_document(&arena, raw_markdown.as_str(), options);

    // Render the Markdown into HTML.
    let mut raw_html = vec![];
    comrak::format_html(markdown_tree, options, &mut raw_html)
        .expect("Error whilst formatting HTML.");
    let raw_html = String::from_utf8(raw_html).expect("Error parsing comrak HTML as UTF-8.");

    Ok(raw_html)
}
