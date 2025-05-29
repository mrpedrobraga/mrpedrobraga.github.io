use std::path::PathBuf;
use {
    crate::render_markdown_simple,
    rocket::{get, routes, Build, Rocket},
};
use {
    comrak::nodes::Ast,
    rocket_dyn_templates::{context, Template},
    std::cell::RefCell,
};
use {
    rocket::response::status::NotFound,
    serde::{Deserialize, Serialize},
};

pub fn mount_routes(ro: Rocket<Build>) -> Rocket<Build> {
    ro.mount("/software", routes![software, home])
}

#[get("/")]
fn home() -> Template {
    Template::render(
        "base",
        context! {
            title: "Tools & Software",
            gimmick_path: "~/software",
            path: "/software",
            nav_index: 2,
            content: render_markdown_simple(PathBuf::from("./pages/software/index.html")).expect("Failed to get file etc")
        },
    )
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct SoftwareFrontmatter {
    title: String,
    description: String,
    tags: Vec<String>,
}

#[get("/<software>")]
fn software(software: &str) -> Template {
    let path = PathBuf::from("./pages/software/")
        .join(software)
        .join("index.md");

    render_software_page(path).expect("Failed rendering markdown file!")
}

fn render_software_page(path: PathBuf) -> Result<Template, NotFound<String>> {
    // Retrive the raw text.
    let raw_markdown = std::fs::read_to_string(path.as_path())
        .map_err(|_| rocket::response::status::NotFound("Not found.".to_owned()))?;

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
            .tasklist(true)
            .alerts(true)
            .strikethrough(true)
            .build(),
        parse: comrak::ParseOptions::builder()
            .default_info_string("rust".to_owned())
            .build(),
        render: comrak::RenderOptions::builder().unsafe_(true).build(),
    };
    let root = comrak::parse_document(&arena, raw_markdown.as_str(), options);

    let mut software_metadata = SoftwareFrontmatter::default();
    let mut math_nodes = vec![];
    let mut fountain_nodes = vec![];

    for node in root.descendants() {
        // Find the blog's metadata.
        if let comrak::nodes::NodeValue::FrontMatter(ref front_matter) = node.data.borrow().value {
            let yaml = front_matter.replace("---", "");
            let front_matter: Result<SoftwareFrontmatter, _> = serde_yaml::from_str(yaml.as_str());
            match front_matter {
                Ok(front_matter) => {
                    software_metadata = front_matter;
                }
                Err(err) => {
                    dbg!(err.to_string());
                }
            }
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
                node.data.borrow().sourcepos.start,
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
                node.data.borrow().sourcepos.start,
            )));
            let new_node = arena.alloc(new_node);
            node.insert_after(new_node);
            node.detach();
        }
    }

    // Render the Markdown into HTML.
    let mut raw_html = vec![];
    comrak::format_html(root, options, &mut raw_html).expect("Error whilst formatting HTML.");
    let raw_html = String::from_utf8(raw_html).expect("Error parsing comrak HTML as UTF-8.");

    // Render the blog template with the inserted content.
    Ok(Template::render(
        "software",
        context! {
            title: software_metadata.title,
            description: software_metadata.description,
            filename: path,
            tags: software_metadata.tags,
            content: raw_html
        },
    ))
}
