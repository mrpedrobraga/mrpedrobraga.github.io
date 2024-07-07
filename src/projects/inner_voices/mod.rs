use axum::{routing::get, Router};
use maud::{html, Markup};

pub fn install_main_module(router: Router) -> Router {
    router.route("/inner-voices", get(main_page))
}

pub async fn main_page() -> Markup {
    html! {
        "Inner Voices!"
    }
}
