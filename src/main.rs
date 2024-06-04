pub mod pages;
pub mod rebuild;

use std::path::PathBuf;

use axum::{extract::Path as RoutePath, routing::get, Router};
use maud::Markup;
use pages::{docs::docs_page, not_found_page};
use rebuild::rebuild_markdown_directory_recursive;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let _ = rebuild_dist()?;

    let routes = Router::<()>::new()
        .route("/docs/*path", get(get_docs))
        .nest_service("/public", ServeDir::new("public"));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4321")
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
    Ok(())
}

async fn get_docs(RoutePath(path): RoutePath<String>) -> Markup {
    let docs_dir_path = std::path::Path::new("./dist/docs");

    let docs_path = PathBuf::from(docs_dir_path)
        .join(path)
        .with_extension("html");
    let content_html = std::fs::read_to_string(docs_path);

    content_html
        .map(|raw_html| docs_page(raw_html))
        .unwrap_or(not_found_page())
}

fn rebuild_dist() -> miette::Result<()> {
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
