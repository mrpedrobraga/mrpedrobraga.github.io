use std::path::PathBuf;

use clap::Parser;
use rocket::{
    catchers,
    fs::{relative, FileServer},
    routes,
};

use self::projects::project_page_html;

pub mod api;
pub mod home;
pub mod inner_voices;
pub mod markdown;

pub mod projects;

#[allow(clippy::result_large_err)]
#[rocket::main]
async fn main() -> Result<(), rocket::error::Error> {
    let args = CliArgs::parse();

    match args {
        CliArgs::Build => {
            build_website().await;
            Ok(())
        }
        CliArgs::Dev => {
            build_website().await;
            serve_website(false).await?;
            Ok(())
        }
        CliArgs::Prod => {
            //build_website().await;
            serve_website(true).await?;
            Ok(())
        }
    }
}

#[derive(clap::Parser)]
enum CliArgs {
    Build,
    Dev,
    Prod,
}

async fn build_website() {
    let dist_directory = PathBuf::from("./dist");

    let _ = std::fs::create_dir(&dist_directory);
    std::fs::write(
        dist_directory.join("index.html"),
        home::home_html().to_string(),
    )
    .unwrap();
    let _ = std::fs::create_dir(dist_directory.join("projects"));
    std::fs::write(
        dist_directory.join("projects/index.html"),
        projects::project_list_html().to_string(),
    )
    .unwrap();

    let projects_dir = std::fs::read_dir("./content/pages/projects").unwrap();

    for entry in projects_dir {
        let entry = entry.unwrap();
        let e_name = entry.file_name();
        let e_name = e_name.to_str().unwrap();

        let html = project_page_html(e_name.to_owned());

        let _ = std::fs::create_dir(dist_directory.join(format!("projects/{e_name}/")));
        std::fs::write(
            dist_directory.join(format!("projects/{e_name}/index.html")),
            html.to_string(),
        )
        .unwrap();
    }
}

pub struct WebsiteContext {
    pub dist_dir: PathBuf
}

#[allow(clippy::result_large_err)]
async fn serve_website(prod: bool) -> Result<(), rocket::Error> {
    let dist_dir = PathBuf::from(if prod { "/app/dist" } else { "./dist" });
    let dd2 = dist_dir.clone();
    let public_dir = PathBuf::from(if prod { "/app/public" } else { "./public" });

    let mut rocket_app = rocket::build()
        .manage(WebsiteContext { dist_dir })
        .mount("/public", FileServer::from(public_dir))
        .register("/", catchers![home::not_found])
        .mount("/", routes![home::home])
        .mount("/projects", FileServer::from(dd2.join("projects")));
    // .mount("/md-projects", routes![projects::projects])
    // .mount("/md-project", routes![projects::project]);

    rocket_app = api::mount_routes(rocket_app);
    rocket_app = inner_voices::mount_routes(rocket_app);
    rocket_app.launch().await?;

    Ok(())
}
