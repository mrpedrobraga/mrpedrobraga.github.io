use plait::{Html, ToHtml};
use rocket::{
    catch,
    fs,
    get,
};
use std::path::PathBuf;

use crate::markdown;

#[get("/")]
pub fn home() -> Html {
    let rendered_file_res =
        markdown::render_from_path_full::<()>(PathBuf::from("./content/pages/index.md"));
    let Ok(rendered_file) = rendered_file_res else {
        return not_found();
    };

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
                    nav {
                        "Navigation goes here..."
                    }
                }
                main(class: "compact") {
                    #(rendered_file.html_content)
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
