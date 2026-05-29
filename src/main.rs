use plait::{Html, ToHtml};
use rocket::{
    catch, catchers,
    fs::{relative, FileServer},
    get, launch, routes,
};
use std::path::PathBuf;

pub mod api;
pub mod inner_voices;
pub mod markdown;

pub mod projects;

#[launch]
fn rocket() -> _ {
    let mut rocket_app = rocket::build()
        .mount("/public", FileServer::from(relative!("public")))
        .register("/", catchers![not_found])
        .mount("/", routes![index])
        .mount("/projects", routes![projects::projects])
        .mount("/project", routes![projects::project]);

    rocket_app = api::mount_routes(rocket_app);
    rocket_app = inner_voices::mount_routes(rocket_app);

    rocket_app
}

#[get("/")]
fn index() -> Html {
    let rendered_file_res =
        markdown::render_from_path_full::<()>(PathBuf::from("./content/pages/index.md"));
    let Ok(rendered_file) = rendered_file_res else {
        return not_found();
    };

    plait::html! {
        #doctype
        html {
            head {
                title { "Index" }
                link (rel: "stylesheet", href: "/public/styles/main.css");
            }
            body {
                header {
                    nav {
                        "Navigation goes here..."
                    }
                }
                main {
                    #(rendered_file.html_content)
                }
            }
        }
    }
    .to_html()
}

#[catch(404)]
fn not_found() -> Html {
    plait::html! {
        "Not found"
    }
    .to_html()
}
