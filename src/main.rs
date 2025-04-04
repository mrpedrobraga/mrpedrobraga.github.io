use api::{api_catchers, api_routes};
use comrak::nodes::{Ast, LineColumn};
use comrak::{
    arena_tree::Node,
    nodes::{NodeHtmlBlock, NodeValue},
    Arena, ExtensionOptions, ParseOptions, RenderOptions,
};
use latex2mathml::DisplayStyle;
use rocket::{
    catch, catchers,
    fs::{relative, FileServer},
    get, launch,
    response::status::NotFound,
    routes,
    serde::{Deserialize, Serialize},
};
use rocket_dyn_templates::{context, Template};
use std::cell::RefCell;
use std::path::Path;

pub mod api;
pub mod inner_voices;

#[launch]
fn rocket() -> _ {
    let mut ro = rocket::build()
        .mount("/public", FileServer::from(relative!("public")))
        .mount("/blog/assets", FileServer::from(relative!("blog/assets")))
        .register("/", catchers![not_found])
        .mount("/", routes![index])
        .mount("/api", api_routes())
        .register("/api", api_catchers())
        .attach(Template::fairing());

    ro = inner_voices::mount_routes(ro);

    ro
}

#[get("/")]
fn index() -> Template {
    Template::render("base", context! {})
}

#[derive(Serialize, Deserialize, Default)]
struct BlogFrontmatter {
    title: String,
    date: String,
    description: String,
    tags: Vec<String>,
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
    let mut blog_metadata = BlogFrontmatter::default();

    let mut aoi = vec![];

    for node in root.descendants() {
        if let NodeValue::FrontMatter(ref front_matter) = node.data.borrow().value {
            let yaml = front_matter.replace("---", "");
            println!("Frontmatter YAML: {}", yaml);
            let front_matter: BlogFrontmatter =
                serde_yaml::from_str(yaml.as_str()).expect("Failed to parse the frontmatter!");
            blog_metadata = front_matter;
        }

        if let NodeValue::Math(_) = node.data.borrow().value {
            aoi.push(node);
        }
    }

    for node in aoi {
        if let NodeValue::Math(ref latex) = node.data.borrow().value {
            let display_style = if latex.display_math {
                DisplayStyle::Block
            } else {
                DisplayStyle::Inline
            };
            let mathml = latex2mathml::latex_to_mathml(latex.literal.as_str(), display_style)
                .unwrap_or("[Invalid math]".into());
            let new_node = Node::new(RefCell::new(Ast::new(
                NodeValue::HtmlBlock(NodeHtmlBlock {
                    block_type: 0,
                    literal: mathml,
                }),
                LineColumn::from(node.data.borrow().sourcepos.start.clone()),
            )));
            let new_node = arena.alloc(new_node);
            node.insert_after(new_node);
            node.detach();
        }
    }

    let mut raw_html = vec![];
    comrak::format_html(&root, &options, &mut raw_html).expect("Error whilst formatting HTML.");
    let raw_html = String::from_utf8(raw_html).expect("Error parsing comrak HTML as UTF-8.");

    Ok(Template::render(
        "blog",
        context! {
            title: blog_metadata.title,
            description: blog_metadata.description,
            date: blog_metadata.date,
            filename: article,
            content: raw_html
        },
    ))
}

#[catch(404)]
fn not_found() -> String {
    "Not found.".into()
}
