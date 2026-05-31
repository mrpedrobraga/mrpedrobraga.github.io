use std::path::PathBuf;

use clap::Parser;
use rocket::{
    catchers,
    fs::{relative, FileServer},
    launch, routes,
};

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
            println!("Building...");

            let dist_directory = PathBuf::from("./dist");
            std::fs::create_dir(dist_directory.as_path()).unwrap();

            Ok(())
        }
        CliArgs::Serve => {
            println!("Serving from /dist directory...");

            let mut rocket_app = rocket::build()
                .mount("/public", FileServer::from(relative!("public")))
                .register("/", catchers![home::not_found])
                .mount("/", routes![home::home])
                .mount("/projects", routes![projects::projects])
                .mount("/project", routes![projects::project]);

            rocket_app = api::mount_routes(rocket_app);
            rocket_app = inner_voices::mount_routes(rocket_app);
            rocket_app.launch().await?;

            Ok(())
        }
    }
}

#[derive(clap::Parser)]
enum CliArgs {
    Build,
    Serve,
}
