use actix_web::Result as AwResult;
use actix_web::get;
use maud::{DOCTYPE, html};

pub mod about;
pub mod home;
pub mod how_it_works;

#[get("/")]
pub async fn index() -> AwResult<maud::Markup> {
    Ok(render_layout(&maud::html! {}))
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
            body class="w-100 h-100 sans-serif ma0 overflow-hidden" {
                nav class="dt w-100 bg-black-80" {
                    div class="dtc v-mid pa3" {
                        a href="/" class="link white-90 hover-white no-underline fw6 f4" {
                            "CHTMX"
                        }
                    }
                    div class="dtc v-mid tr pa3" {
                        a class="f6 fw4 hover-white no-underline white-70 dn dib-ns pv2 ph3"
                          href="/ui/how-it-works"
                          hx-get="/ui/how-it-works"
                          hx-target="#feature"
                          hx-swap="innerHTML"
                          hx-push-url="true" { "How it Works" }
                        a class="f6 fw4 hover-white no-underline white-70 dn dib-l pv2 ph3"
                          href="/ui/about"
                          hx-get="/ui/about"
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
                main id="feature" class="tc ph3 pv4" style="height: calc(100vh - 4rem); overflow-y: auto;" {
                    (main_content)
                }
            }
        }
    }
}
