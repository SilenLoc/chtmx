use actix_web::Result as AwResult;
use actix_web::{HttpRequest, get};
use maud::{DOCTYPE, html};

pub mod about;
pub mod databases;
pub mod home;
pub mod how_it_works;
pub mod upload;

#[get("/")]
pub async fn index(req: HttpRequest) -> AwResult<maud::Markup> {
    let content = html! {
        div class="tl" {
            h1 class="f2 fw6 white mb2 lh-title" { "CHTMX" }
            p class="f5 white-70 mt2 mb4 lh-copy" { 
                "Testing the capabilities of ClickHouse, HTMX, and Rust"
            }
            
            h2 class="f4 fw6 white-90 mt4 mb3 bb b--white-20 pb2" { "Features" }
            
            div class="flex flex-wrap nl2 nr2" {
                // Databases Panel
                a href="/databases"
                  hx-get="/databases"
                  hx-target="#feature"
                  hx-swap="innerHTML"
                  hx-push-url="true"
                  class="no-underline w-50-ns w-100 pa2" {
                    div class="bg-white-10 white hover-bg-white-20 br2 pa3 h-100" {
                        div class="f3 mb2" { "🗄️" }
                        h3 class="f5 fw6 mb2 white" { "Databases" }
                        p class="f6 white-60 lh-copy ma0" { "Browse ClickHouse databases and tables" }
                    }
                }
                
                // Upload CSV Panel
                a href="/upload"
                  hx-get="/upload"
                  hx-target="#feature"
                  hx-swap="innerHTML"
                  hx-push-url="true"
                  class="no-underline w-50-ns w-100 pa2" {
                    div class="bg-white-10 white hover-bg-white-20 br2 pa3 h-100" {
                        div class="f3 mb2" { "📤" }
                        h3 class="f5 fw6 mb2 white" { "Upload CSV" }
                        p class="f6 white-60 lh-copy ma0" { "Import CSV files into ClickHouse" }
                    }
                }
                
                // How it Works Panel
                a href="/how-it-works"
                  hx-get="/how-it-works"
                  hx-target="#feature"
                  hx-swap="innerHTML"
                  hx-push-url="true"
                  class="no-underline w-50-ns w-100 pa2" {
                    div class="bg-white-10 white hover-bg-white-20 br2 pa3 h-100" {
                        div class="f3 mb2" { "⚙️" }
                        h3 class="f5 fw6 mb2 white" { "How it Works" }
                        p class="f6 white-60 lh-copy ma0" { "Learn about the tech stack" }
                    }
                }
                
                // About Panel
                a href="/about"
                  hx-get="/about"
                  hx-target="#feature"
                  hx-swap="innerHTML"
                  hx-push-url="true"
                  class="no-underline w-50-ns w-100 pa2" {
                    div class="bg-white-10 white hover-bg-white-20 br2 pa3 h-100" {
                        div class="f3 mb2" { "ℹ️" }
                        h3 class="f5 fw6 mb2 white" { "About" }
                        p class="f6 white-60 lh-copy ma0" { "More information about this project" }
                    }
                }
            }
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
            body class="w-100 sans-serif ma0 bg-black white" style="height: 100vh; overflow: hidden;" {
                nav class="dt w-100 bg-black bb b--white-20 fixed top-0 left-0 right-0 z-1" {
                    div class="dtc v-mid pa3" {
                        a href="/" class="link white-90 hover-white no-underline fw6 f4" {
                            "CHTMX"
                        }
                    }
                    div class="dtc v-mid tr pa3" {
                        a class="f6 fw4 hover-white no-underline white-70 dn dib-ns pv2 ph3"
                          href="/databases"
                          hx-get="/databases"
                          hx-target="#feature"
                          hx-swap="innerHTML"
                          hx-push-url="true" { "Databases" }
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
                main id="feature" class="flex flex-column" style="padding-top: 5rem; padding-left: 10px; padding-right: 10px; padding-bottom: 10px; height: 100vh; overflow: hidden;" {
                    div class="w-100 flex-auto" style="overflow: hidden; display: flex; flex-direction: column;" {
                        (main_content)
                    }
                }
            }
        }
    }
}
