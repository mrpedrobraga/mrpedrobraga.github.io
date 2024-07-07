use axum::{routing::get, Router};
use maud::{html, Markup};

pub fn install_main_module(router: Router) -> Router {
    router
        .route("/", get(main_page))
        .fallback(get(not_found_page))
}

pub fn terminal_page(content: Markup) -> Markup {
    html! {
        head {
            link href="./public/reset.css" rel="stylesheet";
            link href="./public/main.css" rel="stylesheet";
        }
        body {
            header {
                div class="window terminal" {
                    div class="titlebar" {
                        "profile.sol"
                    }
                    div id="profile" {
                        img src="./public/images/me.jpg" id="profile_picture";
                        pre {(include_str!("./profile.sol"))}
                    }
                }
            }
            main {(content)}
        }
    }
}

pub async fn main_page() -> Markup {
    terminal_page(html! {
        pre { "Butterfly Linux (C) 2024 All Rights Reserved"}
        pre { "Version 0.0.1 (tty1)" br; br; }

        pre {span {"~"} span class="green" {"$"} span class="faded" {" ls ."}}
        (navigation())
        br;
        pre {span {"~"} span class="green" {"$"} span class="faded" {" cat info.txt"}}
        p {
            "Hello stranger, welcome to my web site, the only corner of the whole world wide web that is mine. "
            " Feel free to inspect my files but... do so responsibly. You see..."
            br; br;
            " I was told to make a web site using Hypertext (what a pretentious name, by the way) and style sheets."
            " Now, I do see the reason why some laypeople would use 'documents' to share their things on the internet."
            " But a true computer nerd(TM) would promptly observe the limitation of the markup language and 'JavaScript' for reliable software development."
            br;br;
            " So I just exposed this old little machine to the open internet. Yeah, just click on the directories and files to nagivate."
            " One day the internet will be big, and so will I (metaphorically) (on the internet)."
        }
        br;
        pre {span {"~"} span class="green" {"$"} div class="caret" {}}
    })
}

fn navigation() -> Markup {
    html! {
        nav {
            a class="dir" href="." {
                "."
            }
            a class="dir" href=".." onclick="alert('where are you going this is the homepage dude')" {
                ".."
            }
            a class="dir" href="./art" {
                "/art"
            }
            a class="dir" href="./game-design" {
                "/game-design"
            }
            a class="dir" href="./sol" {
                "/sol"
            }
            a class="dir" href="./inner-voices" {
                "/inner-voices"
            }
            a class="dir" href="./music" {
                "/music"
            }
            a { "info.txt" }
        }
    }
}

pub async fn not_found_page(req: axum::extract::Request) -> Markup {
    terminal_page(html! {
        pre {span {"~"} span class="green" {"$"} span class="faded" {(format!(" cd ~{:?}", req.uri()))}}
        (format!("bash: cd: {}: No such file or directory.", req.uri()))
        br;
        br;
        pre {span {"~"} span class="green" {"$"} span class="faded" {" ls ."}}
        (navigation())
        br;
        pre {span {"~"} span class="green" {"$"} div class="caret" {}}
    })
}
