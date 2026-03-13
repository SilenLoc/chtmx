use actix_web::Result as AwResult;
use actix_web::{HttpRequest, get};
use maud::{DOCTYPE, html};

pub mod about;
pub mod home;
pub mod how_it_works;
pub mod upload;

#[get("/")]
pub async fn index(req: HttpRequest) -> AwResult<maud::Markup> {
    let content = html! {
        div {
            h1 class="f2 f1-l fw2 white-90 mb0 lh-title" { "This is your super impressive headline" }
            h2 class="fw1 f3 white-80 mt3 mb4" { "Now a subheadline where explain your wonderful new startup even more" }
            a class="f6 no-underline grow dib v-mid bg-blue white ba b--blue ph3 pv2 mb3" href="/" { "Call to Action" }
            span class="dib v-mid ph3 white-70 mb3" { "or" }
            a class="f6 no-underline grow dib v-mid white ba b--white ph3 pv2 mb3" href="" { "Secondary call to action" }
        }
    };

    // Check if this is an htmx request
    if req.headers().get("HX-Request").is_some() {
        Ok(content)
    } else {
        Ok(render_layout(&content))
    }
}

pub fn render_layout(main_content: &maud::Markup) -> maud::Markup {
    html! {
        (DOCTYPE)
        html class="h-100" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "CHTMX" }
                link rel="stylesheet" href="/assets/t.css";
                script src="/assets/h.js" {}
            }
            body class="w-100 min-h-100 sans-serif ma0" {
                nav class="dt w-100 bg-black-80 fixed top-0 left-0 right-0 z-1" {
                    div class="dtc v-mid pa3" {
                        a href="/" class="link white-90 hover-white no-underline fw6 f4" {
                            "CHTMX"
                        }
                    }
                    div class="dtc v-mid tr pa3" {
                        a class="f6 fw4 hover-white no-underline white-70 dn dib-ns pv2 ph3"
                          href="/upload"
                          hx-get="/upload"
                          hx-target="#feature"
                          hx-swap="innerHTML"
                          hx-push-url="true" { "Upload CSV" }
                        a class="f6 fw4 hover-white no-underline white-70 dn dib-ns pv2 ph3"
                          href="/how-it-works"
                          hx-get="/how-it-works"
                          hx-target="#feature"
                          hx-swap="innerHTML"
                          hx-push-url="true" { "How it Works" }
                        a class="f6 fw4 hover-white no-underline white-70 dn dib-l pv2 ph3"
                          href="/about"
                          hx-get="/about"
                          hx-target="#feature"
                          hx-swap="innerHTML"
                          hx-push-url="true" { "About" }
                        div class="dib v-mid ml3"
                            hx-get="/health/db/status"
                            hx-trigger="load"
                            hx-swap="innerHTML" {
                            span class="white-70 f6" { "Loading..." }
                        }
                    }
                }
                div class="fixed top-0 left-0 right-0 bottom-0 cover bg-center" style="background-image: url(/assets/background.jpg); z-index: -1;" {
                    div class="bg-black-10 h-100" {}
                }
                main id="feature" class="tc ph3 pv4 flex items-center justify-center" style="padding-top: 5rem; min-height: 100vh;" {
                    div class="w-100" {
                        (main_content)
                    }
                }
            }
        }
    }
}
