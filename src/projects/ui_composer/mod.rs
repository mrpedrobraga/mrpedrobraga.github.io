use std::path::PathBuf;

use axum::{extract::Path as RoutePath, routing::get, Router};
use maud::{html, Markup};

use crate::{
    pages::{docs::docs_page, not_found_page},
    rebuild::rebuild_markdown_directory_recursive,
};

pub fn install_ui_composer_module(router: Router) -> Router {
    //let _ = rebuild_sol_documentation();

    router.route("/ui-composer", get(get_sol_homepage))
}

fn navigation() -> Markup {
    html! {
        nav {
             a class="dir" href="." {
                 "."
             }
             a class="dir" href=".." {
                 ".."
             }
             a class="dir" href="/ui-composer/docs" {
                 "/docs"
             }
             a { "info.md" }
         }
    }
}

pub async fn get_sol_homepage() -> Markup {
    return html! {
        head {
            title { "guest@mrpedrobraga:~/ui-composer" }
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
                pre {span {"~/ui-composer"} span class="green" {"$"} span class="faded" {" ls ."}}
                (navigation())
                br;
                pre {span {"~/ui-composer"} span class="green" {"$"} span class="faded" {" cat info.md"}}
                pre {
                    (include_str!("./info.md"))
                }
                br;
                pre {span {"~/ui-composer"} span class="green" {"$"} span class="faded" {" ls ."}}
                (navigation())
                br;
                pre {span {"~"} span class="green" {"$"} div class="caret" {}}
            }
        }
    };
}
