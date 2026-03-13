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
                p class="f4 mb3" {
                    "Hi, I'm "
                    a class="white-90 hover-white" href="https://me.silenlocatelli.ch/" target="_blank" { "Silen Locatelli" }
                    "."
                }

                p class="f5 mb3" { "Clean code is tested code, right?" }

                h2 class="f3 fw6 white-90 mb3 mt4" { "What I Do" }
                p class="f5 mb3" {
                    "I code in my freetime and at work. I organize "
                    a class="white-90 hover-white" href="https://rust-basel.ch/" target="_blank" { "Rust-Basel" }
                    " meetups and give talks about API testing, HTMX, and Rust development."
                }

                h2 class="f3 fw6 white-90 mb3 mt4" { "Recent Talks" }
                ul class="f5 list pl3" {
                    li class="mb2" {
                        a class="white-90 hover-white" href="https://github.com/SilenLoc/baselOne2024" target="_blank" {
                            "I like my API tests raw"
                        }
                        " - BaselOne 2024"
                    }
                    li class="mb2" {
                        a class="white-90 hover-white" href="https://github.com/rust-basel/htmx-workshop-meetup-10" target="_blank" {
                            "HTMX with Rust"
                        }
                        " - Rust Basel Workshop"
                    }
                    li class="mb2" {
                        a class="white-90 hover-white" href="https://github.com/rust-basel/cli-meetup-12" target="_blank" {
                            "Your first (Rust) CLI project"
                        }
                        " - Rust Basel Meetup"
                    }
                }

                h2 class="f3 fw6 white-90 mb3 mt4" { "Projects" }
                ul class="f5 list pl3" {
                    li class="mb2" {
                        a class="white-90 hover-white" href="https://silenloc.github.io/TypeFast/" target="_blank" {
                            "TypeFast"
                        }
                        " - A typing practice tool with no lags"
                    }
                    li class="mb2" {
                        a class="white-90 hover-white" href="https://blog.silenlocatelli.ch/" target="_blank" {
                            "My Blog"
                        }
                    }
                    li class="mb2" {
                        a class="white-90 hover-white" href="https://jobs.silenlocatelli.ch/" target="_blank" {
                            "Job Scraper"
                        }
                        " - A job post scraper"
                    }
                }

                h2 class="f3 fw6 white-90 mb3 mt4" { "Find Me" }
                div class="f5" {
                    p class="mb2" {
                        "📍 Basel, Switzerland"
                    }
                    p class="mb2" {
                        "🏢 Optravis LLC"
                    }
                    p class="mb2" {
                        a class="white-90 hover-white" href="https://github.com/SilenLoc" target="_blank" { "GitHub" }
                        " · "
                        a class="white-90 hover-white" href="https://www.linkedin.com/in/silen-locatelli" target="_blank" { "LinkedIn" }
                        " · "
                        a class="white-90 hover-white" href="https://buymeacoffee.com/silenloc" target="_blank" { "Buy me a coffee" }
                    }
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
