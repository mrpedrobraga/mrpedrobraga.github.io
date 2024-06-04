use maud::{html, Markup, PreEscaped};

pub fn docs_page(content_html: String) -> Markup {
    html! {
        head {
            link rel="stylesheet" href="../../../public/main.css" {}
        }
        body {
            div .flex_h {
                div .flex_centered {
                    aside #sidebar .left {
                        header {
                            h1 .small_title {
                                ("☀️Sol")
                            }
                        }
                        h4 { "Getting Started" }
                        ul {
                            li { "Installing" }
                            li { "The CLI" }
                            li { "Integration" }
                            li { "Your first asset" }
                            li { "Installing Dependencies" }
                        }
                        h4 { "Reference" }
                        ul {
                            li { "Data" }
                        }
                    }
                    main {
                        (PreEscaped(content_html))
                        div .spacer {}
                    }
                    aside #sidebar {

                    }
                }
            }
        }
    }
}
