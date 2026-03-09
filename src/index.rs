use actix_web::Responder;
use maud::{DOCTYPE, html};

pub async fn index() -> impl Responder {
    let markup = html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "CHTMX" }
                link rel="stylesheet" href="/assets/t.css";
                script src="/assets/h.js" {}
            }
            body class="sans-serif pa4" {
                div class="mw7 center" {
                    h1 class="f2 mb3" { "Welcome to CHTMX" }
                    p class="f4 lh-copy" {
                        "This is a simple HTMX application using Tachyons CSS and Maud templates."
                    }
                }
            }
        }
    };
    maud::PreEscaped(markup.into_string())
}
