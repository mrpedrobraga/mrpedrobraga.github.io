use crate::{
    markdown::{self, FromFrontmatter, get_frontmatter, syntax_highlighting::syntax_highlighting_for_html},
};
use plait::{Html, ToHtml as _};
use rocket::get;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFrontmatter {
    title: String,
    description: String,
    tags: Vec<String>,
    link: Option<String>,
    banner_url: Option<String>,
}

impl FromFrontmatter for ProjectFrontmatter {
    fn new() -> Self {
        ProjectFrontmatter {
            title: String::from("Untitled Project"),
            description: String::from("Missing description"),
            ..Default::default()
        }
    }
}

#[get("/")]
pub fn projects() -> Html {
    // TODO: Precompute this...
    let mut project_list = std::fs::read_dir("./content/pages/projects")
        .unwrap()
        .filter_map(|dir_entry| {
            let dir_entry = dir_entry.unwrap();
            let dir_name = dir_entry.file_name().into_string().unwrap();
            let dir_index_file_path = dir_entry.path().join("index.md");
            let raw_markdown = std::fs::read_to_string(dir_index_file_path).ok().unwrap();
            let frontmatter: Option<ProjectFrontmatter> = get_frontmatter(raw_markdown.as_str());
            frontmatter.map(|f| (dir_name, f))
        })
        .collect::<Vec<(String, ProjectFrontmatter)>>();

    project_list.sort_by(|(_, a), (_, b)| a.title.cmp(&b.title));

    let directories = plait::html! {
        for (dir_name, project) in project_list.iter() {
            div (class: "project-list-entry") {
                h2 { a(href: format!("/project/{dir_name}")) { (project.title) } }
                span { (project.description) }
                div (class: "tags") {
                    for tag in project.tags.iter() {
                        div(class: "tag") { (tag) }
                    }
                }
            }
        }
    };

    plait::html! {
        #doctype
        html {
            head {
                title { "Projects" }
                link (rel: "stylesheet", href: "/public/styles/main.css");
            }
            body {
                header {
                    nav {
                        div (class: "inner") {
                            "Nav goes here"
                        }
                    }
                }
                main {
                    div (class: "project-listing") {
                        (directories)
                    }
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

#[get("/<project_name>")]
pub fn project(project_name: String) -> Html {
    let url = format!("./content/pages/projects/{project_name}/index.md");
    let rendered_file_res =
        markdown::render_from_path_full::<ProjectFrontmatter>(PathBuf::from(url));

    let Ok(rendered_file) = rendered_file_res else {
        return crate::home::not_found();
    };

    let banner_url = rendered_file.frontmatter.banner_url.unwrap_or("https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/1a989815-8731-4b08-8124-db03acb4ada8/di7xmh0-8c21639f-8b54-4c3b-bed0-b50b6fbc8d8b.png?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOjdlMGQxODg5ODIyNjQzNzNhNWYwZDQxNWVhMGQyNmUwIiwiaXNzIjoidXJuOmFwcDo3ZTBkMTg4OTgyMjY0MzczYTVmMGQ0MTVlYTBkMjZlMCIsIm9iaiI6W1t7InBhdGgiOiIvZi8xYTk4OTgxNS04NzMxLTRiMDgtODEyNC1kYjAzYWNiNGFkYTgvZGk3eG1oMC04YzIxNjM5Zi04YjU0LTRjM2ItYmVkMC1iNTBiNmZiYzhkOGIucG5nIn1dXSwiYXVkIjpbInVybjpzZXJ2aWNlOmZpbGUuZG93bmxvYWQiXX0.OWTlOXrVlOcGk3DkNyh2X5etCjhDxJ98O53I6vGzMpU".to_owned());

    let plain_title = rendered_file.frontmatter.title.clone();
    let title = plait::html! {
        if let Some(link) = rendered_file.frontmatter.link.as_ref() {
            a(href: link, target: "_blank") {
                (plain_title.as_str())
            }
        } else {
            (plain_title.as_str())
        }
    };

    let tags = rendered_file.frontmatter.tags;
    let tags = plait::html! {
        div (class: "tags") {
            for tag in tags.iter() {
                div(class: "tag") { (tag) }
            }
        }
    };

    plait::html! {
        #doctype
        html {
            head {
                title { (format!("mrpedrobraga | {}", &rendered_file.frontmatter.title)) }
                link (rel: "stylesheet", href: "/public/styles/main.css");

                (syntax_highlighting_for_html())
            }
            body {
                header {
                    nav {
                        div (class: "inner") {
                            "Navigation goes here..."
                        }
                    }
                    div (class: "banner", style: format!("background-image: url('{banner_url}')")) {

                    }
                    div (class: "page-info") {
                        h1 {
                            (title)
                        }
                        p {
                            (rendered_file.frontmatter.description)
                        }
                        div (class: "tags") {
                            (tags)
                        }
                    }
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
