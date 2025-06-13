pub mod quiz;
use rocket::{catch, catchers, get, routes, Build, Rocket};
use rocket_dyn_templates::{context, Template};
use {rand::seq::IndexedRandom, std::path::PathBuf};

pub fn mount_routes(ro: Rocket<Build>) -> Rocket<Build> {
    ro.mount(
        "/inner-voices",
        routes![home, soundtrack, shop, resources, press_kit],
    )
    .register("/inner-voices", catchers![not_found])
}

#[get("/")]
fn home() -> Template {
    let content = render_markdown_simple(PathBuf::from("./pages/inner-voices/home.md"))
        .expect("Failed to get file...");

    Template::render(
        "inner-voices",
        context! {
            title: "Home",
            path: "/",
            nav_index: 1,
            content: content,
        },
    )
}

#[get("/soundtrack")]
fn soundtrack() -> Template {
    let content = render_markdown_simple(PathBuf::from("./pages/inner-voices/soundtrack.md"))
        .expect("Failed to get file...");

    Template::render(
        "inner-voices",
        context! {
            title: "Soundtrack",
            path: "/soundtrack",
            nav_index: 2,
            content: content,
        },
    )
}

#[get("/shop")]
fn shop() -> Template {
    let content = render_markdown_simple(PathBuf::from("./pages/inner-voices/shop.md"))
        .expect("Failed to get file...");

    Template::render(
        "inner-voices",
        context! {
            title: "Shop",
            path: "/shop",
            nav_index: 3,
            content: content
        },
    )
}

#[get("/resources")]
fn resources() -> Template {
    let content = render_markdown_simple(PathBuf::from("./pages/inner-voices/resources.md"))
        .expect("Failed to get file...");

    Template::render(
        "inner-voices",
        context! {
            title: "Resources",
            path: "/resources",
            nav_index: 4,
            content: content
        },
    )
}

#[get("/press-kit")]
fn press_kit() -> Template {
    let content = render_markdown_simple(PathBuf::from("./pages/inner-voices/press-kit.md"))
        .expect("Failed to get file...");

    Template::render(
        "inner-voices",
        context! {
            title: "Press Kit",
            path: "/press-kit",
            nav_index: 5,
            content: content
        },
    )
}

#[catch(404)]
fn not_found() -> Template {
    Template::render(
        "inner-voices-not-found",
        context! {
            content: random_not_found_quip()
        },
    )
}

fn random_not_found_quip() -> &'static str {
    let options = [
        // TheMysKid
        "* Aint there no such page, love...",
        // Pierre
        "- Meow! (Took a wrong turn, human?)",
        // Claire
        "- Uh, I think you got something wrong...",
        // Andy
        "- Hang in there, you'll find your way back!",
        // Bruno
        "- You got out of bounds, dude!",
        // Rodrick
        "- The heck did you do to end up here?",
        // Erica
        "- Smells like, like, the void.",
        // Evelyn
        "- ERICA!! I found a human(?) in the void!!!",
        // Enzo
        "- So, uh, you're lost, I think.",
        // The Basket Knight
        "- HALT! Humans are *NOT* allowed in the void!",
        // Professor Whynott
        "- ZOINK! You have, it seems, gotten lost!\n- Lost, I say!",
        // Manuel
        "- What? Playing at the 404 page? I don't pay you for that, get outta here!",
        // Rosetta
        "- Oh, dear. Rest here a while then get back home, okay?",
        // The Angel
        "  Be not afraid. It is but a 404 page.",
        // Velmont Kid
        "- Not found.",
    ];
    options.choose(&mut rand::rng()).unwrap()
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
