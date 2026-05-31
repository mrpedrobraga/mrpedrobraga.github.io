use plait::{Html, ToHtml};

pub fn syntax_highlighting_for_html() -> Html {
    plait::html! {
        link(rel: "stylesheet", href: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/atom-one-light.min.css");
        script(src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/highlight.min.js") {}
        script(src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/rust.min.js") {}
        script(src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/ini.min.js") {}
        script(src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/sh.min.js") {}
        script(src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/glsl.min.js") {}
        script(){ "hljs.highlightAll();" }
    }.to_html()
}