use crate::{WebsiteContext, markdown::{self, RenderedFile}};
use plait::{Html, ToHtml};
use rocket::{State, catch, fs::NamedFile, get};
use std::path::PathBuf;

#[get("/")]
pub async fn home(cx: &State<WebsiteContext>) -> NamedFile {
    NamedFile::open(cx.dist_dir.join("index.html")).await.unwrap()
}

pub fn nav() -> Html {
    plait::html! {
        nav {
            span(class: "notice") {
                "This is a temporary page, as the website is being rebuilt.\nI appreciate the understanding."
            }
            ul {
                a(href: "/") { li { "Home" } }
                a(href: "/projects") { li { "Projects" } }
                // a(href: "/gallery") { li { "Gallery" } }
            }
        }
    }.to_html()
}

pub fn home_html() -> Html {
    let _nav = nav();

    let rendered_file_res =
        markdown::render_from_path_full::<()>(PathBuf::from("./content/pages/index.md"));
    let rendered_file = rendered_file_res.unwrap_or(RenderedFile {
        frontmatter: (),
        html_content: "Content not found...".to_owned(),
    });

    plait::html! {
        #doctype
        html {
            head {
                title { "mrpedrobraga" }
                link (rel: "stylesheet", href: "/public/styles/main.css");
                link (rel: "favicon", href: "/public/images/favicon.png");

                link(rel: "stylesheet", href: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/default.min.css");
                script(src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/highlight.min.js") {}
                script(src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/rust.min.js") {}
                script(){ "hljs.highlightAll();" }
            }
            body {
                header {
                    #(_nav)
                }
                main(class: "compact") {
                    #(rendered_file.html_content)
                }
                footer {
                    span {
                        "Made with " em { "HTML" } " by Pedro Braga"
                    }
                }
            }
        }
    }
    .to_html()
}

#[catch(404)]
pub fn not_found() -> Html {
    plait::html! {
        "Not found"
    }
    .to_html()
}
