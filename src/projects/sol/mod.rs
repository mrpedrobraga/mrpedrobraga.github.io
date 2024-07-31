use std::path::PathBuf;

use axum::{extract::Path as RoutePath, routing::get, Router};
use maud::{html, Markup};

use crate::{
    pages::{docs::docs_page, not_found_page},
    rebuild::rebuild_markdown_directory_recursive,
};

pub fn install_sol_module(router: Router) -> Router {
    //let _ = rebuild_sol_documentation();

    router
        .route("/sol", get(get_sol_homepage))
        .route("/sol/docs/*path", get(get_sol_docs))
}

pub async fn get_sol_homepage() -> Markup {
    return html! {
        head {
            title { "guest@mrpedrobraga:~/sol" }
            link href="./public/reset.css" rel="stylesheet";
            link href="./public/main.css" rel="stylesheet";
        }
        body {
            header {
                div class="window terminal" {
                    div class="titlebar" {
                        span class="window_title" {"profile.sol"}
                        div class="button" onclick="alert('you cant get rid of me')" { "🗕" }
                        div class="button" onclick="alert('this is already big enough dude')" { "🗖" }
                        div class="button" onclick="alert('just close the tab, what?')" { "✕" }
                    }
                    div id="profile" {
                        img src="./public/images/me.jpg" id="profile_picture";
                        pre {(include_str!("../me/profile.sol"))}
                    }
                }
            }
            main {
                pre {span {"~/sol"} span class="green" {"$"} span class="faded" {" ls ."}}
                nav {
                    a class="dir" href="." {
                        "."
                    }
                    a class="dir" href=".." {
                        ".."
                    }
                    a class="dir" href="./docs" {
                        "/docs"
                    }
                    a { "info.txt" }
                }
                br;
                pre {span {"~/sol"} span class="green" {"$"} span class="faded" {" cat info.txt"}}
                p {
                    "In order to facilitate the creation of games that are story-focused, with lots of custom data (RPGs, especially like my dear Earthbound or Undertale), I developed '.sol'."
                    " It is a text format not unlike Lua, but for the creation of assets instead of scripting."
                }
                br;
                p {
                    "Please refer to the Sol documentation to learn more about its syntax, technology and use cases."
                }
                br;
                pre {span {"~/sol"} span class="green" {"$"} span class="faded" {" cat example.sol"}}
                pre { code {
                    (include_str!("./example.sol"))
                }}
                br;
                pre {span {"~"} span class="green" {"$"} div class="caret" {}}
            }
        }
    };
}

pub async fn get_sol_docs(RoutePath(path): RoutePath<String>) -> Markup {
    let docs_dir_path = std::path::Path::new("./dist/docs");

    let docs_path = PathBuf::from(docs_dir_path)
        .join(path)
        .with_extension("html");
    let content_html = std::fs::read_to_string(docs_path);

    content_html
        .map(|raw_html| docs_page(raw_html))
        .unwrap_or(not_found_page())
}

pub fn rebuild_sol_documentation() -> miette::Result<()> {
    let content_dir_path = std::path::Path::new("./content/docs");
    let target_dir_path = std::path::Path::new("./dist/docs");

    let dir = std::fs::read_dir(PathBuf::from("./content/docs"));
    dir.unwrap().for_each(|entry| {
        dbg!(entry.unwrap());
    });

    rebuild_markdown_directory_recursive(
        content_dir_path.to_path_buf(),
        target_dir_path.to_path_buf(),
    )?;

    Ok(())
}
