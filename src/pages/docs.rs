use maud::{html, Markup, PreEscaped};

pub fn docs_page(content_html: String) -> Markup {
    html! {
        head {
            link rel="stylesheet" href="../../../../public/docs.css" {}
        }
        body {
            main {
                (PreEscaped(content_html))
                div .spacer {}
            }
        }
    }
}
