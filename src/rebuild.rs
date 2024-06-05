use miette::IntoDiagnostic;
use pulldown_cmark::{Parser, Tag};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
use syntastica::{renderer::HtmlRenderer, ts_runtime::Language};
use syntastica_parsers::{Lang, LanguageSetImpl};

pub fn rebuild_markdown_directory_recursive(
    source: PathBuf,
    target: PathBuf,
) -> miette::Result<()> {
    let source_dir = fs::read_dir(source).into_diagnostic()?;
    fs::create_dir_all(target.as_path()).into_diagnostic()?;
    let _ = source_dir
        .par_bridge()
        .map(|entry| -> miette::Result<()> {
            let entry = entry.into_diagnostic()?;
            if entry.metadata().into_diagnostic()?.is_dir() {
                rebuild_markdown_directory_recursive(
                    entry.path(),
                    target.clone().join(entry.path().file_name().unwrap()),
                )?;
            } else {
                let mut target_file = File::create(
                    target
                        .clone()
                        .join(entry.file_name())
                        .with_extension("html"),
                )
                .unwrap();
                let source_raw_markdown = fs::read_to_string(entry.path()).unwrap();
                let parsed_html: String = md_to_html(source_raw_markdown);
                target_file.write(parsed_html.as_bytes()).unwrap();
            }
            Ok(())
        })
        .collect::<Vec<_>>();

    Ok(())
}

pub fn md_to_html<S>(source_raw_markdown: S) -> String
where
    S: AsRef<str>,
{
    let md_events = MarkdownEvents::new(pulldown_cmark::Parser::new(source_raw_markdown.as_ref()));
    let mut result = String::new();
    pulldown_cmark::html::push_html(&mut result, md_events);
    return result;
}

struct MarkdownEvents<'a> {
    parser: Parser<'a>,
}

impl<'a> MarkdownEvents<'a> {
    pub fn new(parser: Parser<'a>) -> Self {
        Self { parser }
    }
}

impl<'a> Iterator for MarkdownEvents<'a> {
    type Item = pulldown_cmark::Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.parser.next()?;

        match next {
            pulldown_cmark::Event::Start(Tag::CodeBlock(
                pulldown_cmark::CodeBlockKind::Fenced(ref language_tag),
            )) => {
                let mut code_buffer = String::new();
                while let Some(pulldown_cmark::Event::Text(text)) = self.parser.next() {
                    code_buffer.push_str(text.as_ref())
                }

                let output = syntastica::highlight(
                    code_buffer,
                    get_language_by_tag(language_tag.as_ref()),
                    &mut LanguageSetImpl::new(),
                    &mut HtmlRenderer::new(),
                    syntastica_themes::catppuccin::latte(),
                )
                .unwrap_or_else(|err| panic!("highlighting failed: {err}"));

                let formatted = output;

                Some(pulldown_cmark::Event::Html(
                    format!("<pre><code>{}</code></pre>", formatted).into(),
                ))
            }
            _ => Some(next),
        }
    }
}

fn get_language_by_tag(tag: &str) -> Lang {
    match tag {
        "json" => Lang::Json,
        _ => Lang::Lua,
    }
}
