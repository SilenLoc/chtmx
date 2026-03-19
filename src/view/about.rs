use actix_web::Result as AwResult;
use actix_web::{HttpRequest, get};
use maud::html;

#[get("/about")]
pub async fn about_page(req: HttpRequest) -> AwResult<maud::Markup> {
    let version = env!("CARGO_PKG_VERSION");
    let content = html! {
        div class="mw7 center" {
            h1 class="f2 f1-l fw2 white-90 mb3 lh-title" { "About" }
            p class="f6 white-60 mb3" { "Version " (version) }

            div class="tl white-80 lh-copy" {
                h2 class="f3 fw6 white-90 mb3 mt4" { "About This Project" }
                p class="f5 mb3" {
                    "CHTMX is an experimental web application exploring the combination of ClickHouse, HTMX, and Rust. "
                    "The project demonstrates server-side rendering with minimal JavaScript, showing how modern web applications "
                    "can be built with excellent performance and developer experience without complex frontend frameworks."
                }
                p class="f5 mb3" {
                    "The goal is to explore patterns for building data-intensive applications where the backend handles "
                    "HTML generation and HTMX manages dynamic interactions. This approach reduces client-side complexity "
                    "while maintaining a responsive user experience."
                }
                p class="f5 mb3" {
                    "Find the source code on "
                    a class="white-90 hover-action-pink" href="https://github.com/SilenLoc/chtmx" target="_blank" { "GitHub" }
                    "."
                }

                h2 class="f3 fw6 white-90 mb3 mt4" { "About Me" }
                p class="f5 mb3" {
                    "I'm Silen Locatelli, a software developer based in Basel, Switzerland. I work at Optravis LLC "
                    "and spend my free time coding, learning new technologies, and contributing to the Rust community."
                }
                p class="f5 mb3" {
                    "I organize the Rust Basel meetup group where we explore Rust programming through workshops and talks. "
                    "My interests include API testing, web development with HTMX, and building developer tools. "
                    "I believe in writing code is what is matters and sharing knowledge with the community."
                }
                p class="f5 mb3" {
                    "Connect with me on "
                    a class="white-90 hover-action-pink" href="https://github.com/SilenLoc" target="_blank" { "GitHub" }
                    " to see more of my projects and experiments."
                }
            }
        }
    };

    // Check if this is an htmx request
    if req.headers().get("HX-Request").is_some() {
        Ok(content)
    } else {
        Ok(super::render_layout(&content))
    }
}
