pub mod pages;
pub mod projects;
pub mod rebuild;

use axum::Router;
use projects::{
    me::install_main_module,
    sol::{install_sol_module, rebuild_sol_documentation},
    ui_composer::install_ui_composer_module,
};
use tower_http::services::ServeDir;
//use tower_livereload::LiveReloadLayer;

const SERVE_ADDR: &'static str = "0.0.0.0";
const SERVE_PORT: &'static str = "8080";

#[tokio::main]
async fn main() -> miette::Result<()> {
    let routes = Some(Router::<()>::new())
        .map(install_main_module)
        .map(install_sol_module)
        .map(install_ui_composer_module)
        .unwrap()
        //.layer(LiveReloadLayer::new())
        .nest_service("/public", ServeDir::new("public"));

    //rebuild_sol_documentation().expect("Could not rebuild sol documentation.");

    let listener = tokio::net::TcpListener::bind(format!("{SERVE_ADDR}:{SERVE_PORT}"))
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
    Ok(())
}
