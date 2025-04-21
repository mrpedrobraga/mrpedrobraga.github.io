use std::{cell::RefCell, path::{Path, PathBuf}};

use comrak::nodes::Ast;
use rocket::{fs::{relative, FileServer}, get, routes, Build, Rocket};
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};

pub fn mount_routes(ro: Rocket<Build>) -> Rocket<Build> {
    ro.mount("/blog", routes![base, blog])
    .mount("/blog/assets", FileServer::from(relative!("blog/assets")))
}

#[get("/")]
fn base() -> Template {
    Template::render(
        "base",
        context! {
            title: "Blog",
            gimmick_path: "~/blog",
            path: "/blog",
            nav_index: 5,
            content: crate::render_markdown_file(PathBuf::from("./blog/blog.md")).expect("Failed to get file etc")
        }
    )
}

#[derive(Serialize, Deserialize, Default)]
struct BlogFrontmatter {
    title: String,
    date: String,
    description: String,
    tags: Vec<String>,
}

#[get("/<article..>")]
fn blog(article: PathBuf) -> Result<Template, rocket::response::status::NotFound<String> > {    
    dbg!(
        Path::new("./blog")
        .join(article.clone())
        .with_extension("md")
        .as_path()
    );
    
    // Retrive the raw text.
    let raw_markdown =
        std::fs::read_to_string(
            Path::new("./blog")
            .join(article.clone())
            .with_extension("md")
            .as_path()
        )
            .map_err(|_| rocket::response::status::NotFound("Not found".into()))?;

    // Parse it indo markdown.
    let arena = comrak::Arena::new();
    let options = &comrak::Options {
        extension: comrak::ExtensionOptions::builder()
            .front_matter_delimiter(String::from("---"))
            .wikilinks_title_after_pipe(true)
            .math_dollars(true)
            .table(true)
            .spoiler(true)
            .footnotes(true)
            .alerts(true)
            .build(),
        parse: comrak::ParseOptions::builder()
            .default_info_string("rust".into())
            .build(),
        render: comrak::RenderOptions::builder().unsafe_(true).build(),
    };
    let root = comrak::parse_document(&arena, raw_markdown.as_str(), options);
    
    let mut blog_metadata = BlogFrontmatter::default();
    let mut math_nodes = vec![];
    let mut fountain_nodes = vec![];

    for node in root.descendants() {
        // Find the blog's metadata.
        if let comrak::nodes::NodeValue::FrontMatter(ref front_matter) = node.data.borrow().value {
            let yaml = front_matter.replace("---", "");
            println!("Frontmatter YAML: {}", yaml);
            let front_matter: BlogFrontmatter =
                serde_yaml::from_str(yaml.as_str()).expect("Failed to parse the frontmatter!");
            blog_metadata = front_matter;
        }

        // Store nodes of interest for parsing LaTeX into MathXML.
        if let comrak::nodes::NodeValue::Math(_) = node.data.borrow().value {
            math_nodes.push(node);
        }

        // Store nodes of interest for parsing fountain scripts
        if let comrak::nodes::NodeValue::CodeBlock(ref c) = node.data.borrow().value {
            if c.info == "fountain" {
                fountain_nodes.push(node);
            }
        }
    }

    // Transform LaTeX into MathXML.
    for node in math_nodes {
        if let comrak::nodes::NodeValue::Math(ref latex) = node.data.borrow().value {
            let display_style = if latex.display_math {
                latex2mathml::DisplayStyle::Block
            } else {
                latex2mathml::DisplayStyle::Inline
            };
            let mathml = latex2mathml::latex_to_mathml(latex.literal.as_str(), display_style)
                .unwrap_or("[Invalid math]".into());
            let new_node = comrak::arena_tree::Node::new(RefCell::new(Ast::new(
                comrak::nodes::NodeValue::HtmlBlock(comrak::nodes::NodeHtmlBlock {
                    block_type: 0,
                    literal: mathml,
                }),
                comrak::nodes::LineColumn::from(node.data.borrow().sourcepos.start.clone()),
            )));
            let new_node = arena.alloc(new_node);
            node.insert_after(new_node);
            node.detach();
        }
    }

    // Transform Fountain into HTML.
    for node in fountain_nodes {
        if let comrak::nodes::NodeValue::CodeBlock(ref code) = node.data.borrow().value {
            let script = fountain::parse_document::<()>(code.literal.as_str());
            let html = script
                .map(|document| {
                    let mut html = document.1.as_html();
                    html.insert_str(4, " class=\"fountain\"");
                    html
                })
                .unwrap_or("[Invalid Fountain]".to_string());
            let new_node = comrak::arena_tree::Node::new(RefCell::new(Ast::new(
                comrak::nodes::NodeValue::HtmlBlock(comrak::nodes::NodeHtmlBlock {
                    block_type: 0,
                    literal: html,
                }),
                comrak::nodes::LineColumn::from(node.data.borrow().sourcepos.start.clone()),
            )));
            let new_node = arena.alloc(new_node);
            node.insert_after(new_node);
            node.detach();
        }
    }

    // Render the Markdown into HTML.
    let mut raw_html = vec![];
    comrak::format_html(&root, &options, &mut raw_html).expect("Error whilst formatting HTML.");
    let raw_html = String::from_utf8(raw_html).expect("Error parsing comrak HTML as UTF-8.");

    // Render the blog template with the inserted content.
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
