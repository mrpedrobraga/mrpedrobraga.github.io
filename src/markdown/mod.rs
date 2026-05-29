use std::{fmt::Debug, path::PathBuf};
use serde::Deserialize;

pub struct RenderedFile<T> {
    pub frontmatter: T,
    pub html_content: String,
}

/// Renders simple markdown from a file.
pub fn render_from_path_simple(
    path: PathBuf,
) -> Result<String, rocket::response::status::NotFound<String>> {
    // Retrive the raw text.
    let raw_markdown = std::fs::read_to_string(path.as_path())
        .map_err(|_| rocket::response::status::NotFound("Not found.".into()))?;

    render_markdown_simple(raw_markdown.as_str())
}

pub fn render_markdown_simple(
    raw_markdown: &str,
) -> Result<String, rocket::response::status::NotFound<String>> {
    // Parse it indo markdown.
    let arena = comrak::Arena::new();
    let options = &comrak::Options {
        extension: comrak::options::Extension::builder().table(true).build(),
        parse: comrak::options::Parse::builder().build(),
        render: comrak::options::Render::builder().r#unsafe(true).build(),
    };
    let markdown_tree = comrak::parse_document(&arena, raw_markdown, options);

    // Render the Markdown into HTML.
    let mut raw_html = String::new();
    comrak::format_html(markdown_tree, options, &mut raw_html)
        .expect("Error whilst formatting HTML.");

    Ok(raw_html)
}

/// Marks that a type can be constructed from frontmatter in one of the .md files...
pub trait FromFrontmatter: Debug + Default + for<'de> Deserialize<'de> {
    fn new() -> Self {
        Self::default()
    }

    fn try_patch_from_str(&mut self, string: &str) {
        let front_matter: Result<Self, _> = serde_yaml::from_str(string);
        match front_matter {
            Ok(front_matter) => {
                *self = front_matter;
            }
            Err(err) => {
                dbg!(err.to_string());
            }
        }
    }
}
impl FromFrontmatter for () {}

pub fn render_from_path_full<Frontmatter: FromFrontmatter>(
    path: PathBuf,
) -> Result<RenderedFile<Frontmatter>, rocket::response::status::NotFound<String>> {
    let raw_markdown = std::fs::read_to_string(path.as_path())
        .map_err(|_| rocket::response::status::NotFound("Not found.".to_owned()))?;
    render_markdown_full::<Frontmatter>(raw_markdown.as_str())
}

pub fn render_markdown_full<Frontmatter: FromFrontmatter>(
    raw_markdown: &str,
) -> Result<RenderedFile<Frontmatter>, rocket::response::status::NotFound<String>> {
    let arena = comrak::Arena::new();
    let options = &comrak::Options {
        extension: comrak::options::Extension::builder()
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
        parse: comrak::options::Parse::builder()
            .default_info_string("rust".to_owned())
            .build(),
        render: comrak::options::Render::builder().r#unsafe(true).build(),
    };
    let root = comrak::parse_document(&arena, raw_markdown, options);
    let mut frontmatter = Frontmatter::new();
    let mut math_nodes = vec![];
    let mut fountain_nodes = vec![];
    for node in root.descendants() {
        if let comrak::nodes::NodeValue::FrontMatter(ref front_matter) = node.data.borrow().value {
            let yaml = front_matter.replace("---", "");
            frontmatter.try_patch_from_str(yaml.as_str());
        }

        if let comrak::nodes::NodeValue::Math(_) = node.data.borrow().value {
            math_nodes.push(node);
        }

        // TODO: Update this to new fenced code block extension format!
        if let comrak::nodes::NodeValue::CodeBlock(ref c) = node.data.borrow().value {
            if c.info == "fountain" {
                fountain_nodes.push(node);
            }
        }
    }
    for node in math_nodes {
        if let comrak::nodes::NodeValue::Math(ref latex) = node.data.borrow().value {
            let display_style = if latex.display_math {
                latex2mathml::DisplayStyle::Block
            } else {
                latex2mathml::DisplayStyle::Inline
            };
            let mathml = latex2mathml::latex_to_mathml(latex.literal.as_str(), display_style)
                .unwrap_or("[Invalid math]".into());
            let new_node =
                comrak::arena_tree::Node::new(std::cell::RefCell::new(comrak::nodes::Ast::new(
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
            let new_node =
                comrak::arena_tree::Node::new(std::cell::RefCell::new(comrak::nodes::Ast::new(
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
    let mut raw_html = String::new();
    comrak::format_html(root, options, &mut raw_html).expect("Error whilst formatting HTML.");
    Ok(RenderedFile { frontmatter, html_content: raw_html })
}

pub fn get_frontmatter<Frontmatter: FromFrontmatter>(
    raw_markdown: &str,
) -> Option<Frontmatter> {
let arena = comrak::Arena::new();
    let options = &comrak::Options {
        extension: comrak::options::Extension::builder()
            .front_matter_delimiter(String::from("---"))
            .build(),
        parse: comrak::options::Parse::builder().build(),
        render: comrak::options::Render::builder().r#unsafe(true).build(),
    };
    let root = comrak::parse_document(&arena, raw_markdown, options);
    let mut frontmatter = Frontmatter::new();
    for node in root.descendants() {
        if let comrak::nodes::NodeValue::FrontMatter(ref front_matter) = node.data.borrow().value {
            let yaml = front_matter.replace("---", "");
            frontmatter.try_patch_from_str(yaml.as_str());
            return Some(frontmatter)
        }
    }

    None
}