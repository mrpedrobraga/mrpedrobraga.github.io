use maud::{html, Markup};

pub mod docs;

pub fn not_found_page() -> Markup {
    html! {
        "Not found."
    }
}
