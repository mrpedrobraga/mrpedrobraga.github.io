use maud::{html, Markup, PreEscaped};

pub fn docs_page(content_html: String) -> Markup {
    html! {
        head {
            link rel="stylesheet" href="../../../public/main.css" {}
        }
        body {
            header {
                h1 .small_title {
                    ("☀️Sol")
                }
            }
            div .flex_h {
                aside #sidebar {
                    h3 { "Reference" }
                    ul {
                        li { "Data" }
                    }
                }
                div .flex_centered {
                    main {
                        (PreEscaped(content_html))
                        div .spacer {}
                    }
                }
            }
        }
    }
}
