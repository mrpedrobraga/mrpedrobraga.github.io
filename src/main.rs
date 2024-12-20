use std::path::Path;

use api::{api_catchers, api_routes};
use comrak::{nodes::NodeValue, Arena, ExtensionOptions, ParseOptions, RenderOptions};
use rocket::{
    catch, catchers,
    fs::{relative, FileServer},
    get, launch,
    response::status::NotFound,
    routes,
    serde::{Deserialize, Serialize},
};
use rocket_dyn_templates::{context, Template};

pub mod api;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/public", FileServer::from(relative!("public")))
        .mount("/blog/assets", FileServer::from(relative!("blog/assets")))
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

#[derive(Serialize, Deserialize)]
struct BlogFrontmatter {
    title: String,
}

#[get("/blog/<article>")]
fn blog(article: &str) -> Result<Template, NotFound<String>> {
    let raw_markdown =
        std::fs::read_to_string(Path::new(format!("./blog/{}.md", article).as_str()))
            .map_err(|_| NotFound(not_found()))?;
    let arena = Arena::new();
    let options = &comrak::Options {
        extension: ExtensionOptions::builder()
            .front_matter_delimiter(String::from("---"))
            .wikilinks_title_after_pipe(true)
            .math_dollars(true)
            .build(),
        parse: ParseOptions::builder()
            .default_info_string(String::from("rust"))
            .build(),
        render: RenderOptions::builder().unsafe_(true).build(),
    };
    let root = comrak::parse_document(&arena, raw_markdown.as_str(), options);
    let mut title = "New Article".to_owned();

    for node in root.descendants() {
        if let NodeValue::FrontMatter(ref front_matter) = node.data.borrow().value {
            let yaml = front_matter.replace("---", "");
            println!("Frontmatter YAML: {}", yaml);
            let front_matter: BlogFrontmatter =
                serde_yaml::from_str(yaml.as_str()).expect("Failed to parse the frontmatter!");
            title = front_matter.title;
        }
    }

    let mut raw_html = vec![];
    comrak::format_html(&root, &options, &mut raw_html).expect("Error whilst formatting HTML.");
    let raw_html = String::from_utf8(raw_html).expect("Error parsing comrak HTML as UTF-8.");

    Ok(Template::render(
        "blog",
        context! { title, content: raw_html },
    ))
}

#[catch(404)]
fn not_found() -> String {
    "Not found.".into()
}
