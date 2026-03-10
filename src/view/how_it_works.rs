use actix_web::Result as AwResult;
use actix_web::{HttpRequest, get};
use maud::html;

#[get("/ui/how-it-works")]
pub async fn how_it_works_page(req: HttpRequest) -> AwResult<maud::Markup> {
    let content = html! {
        div class="mw7" {
            h1 class="f2 f1-l fw2 white-90 mb3 lh-title" { "How It Works" }

            div class="tl white-80 lh-copy" {
                p class="f4 mb4" {
                    "CHTMX is a modern web application built with Rust, HTMX, and Tachyons CSS. "
                    "It demonstrates how to build fast, interactive web applications without heavy JavaScript frameworks."
                }

                h2 class="f3 fw6 white-90 mb3 mt4" { "🦀 Rust Backend" }
                p class="f5 mb3" {
                    "The backend is powered by "
                    span class="fw6" { "Actix-web" }
                    ", a blazingly fast web framework for Rust. It provides:"
                }
                ul class="f5 list pl3 mb4" {
                    li class="mb2" { "⚡ Lightning-fast response times" }
                    li class="mb2" { "🔒 Memory safety without garbage collection" }
                    li class="mb2" { "🚀 Async/await support for concurrent operations" }
                    li class="mb2" { "📊 ClickHouse database integration for analytics" }
                }

                h2 class="f3 fw6 white-90 mb3 mt4" { "✨ HTMX Frontend" }
                p class="f5 mb3" {
                    span class="fw6" { "HTMX" }
                    " allows you to access modern browser features directly from HTML, making it simple to:"
                }
                ul class="f5 list pl3 mb4" {
                    li class="mb2" { "🔄 Update page content without full page reloads" }
                    li class="mb2" { "📡 Make AJAX requests with simple HTML attributes" }
                    li class="mb2" { "🎯 Swap content dynamically using CSS selectors" }
                    li class="mb2" { "📱 Create SPA-like experiences with minimal JavaScript" }
                }

                div class="bg-black-40 br3 pa3 mb4" {
                    p class="f6 white-70 ma0 mb2" { "Example: This navigation uses HTMX!" }
                    pre class="f7 white-90 ma0 overflow-x-auto" {
                        code { "<a href=\"/ui/about\" hx-get=\"/ui/about\" hx-target=\"#feature\" hx-swap=\"innerHTML\">About</a>" }
                    }
                }

                h2 class="f3 fw6 white-90 mb3 mt4" { "🎨 Tachyons CSS" }
                p class="f5 mb3" {
                    span class="fw6" { "Tachyons" }
                    " is a functional CSS framework that provides:"
                }
                ul class="f5 list pl3 mb4" {
                    li class="mb2" { "📦 Small file size (only ~14kb)" }
                    li class="mb2" { "🎯 Composable design system" }
                    li class="mb2" { "📱 Mobile-first responsive design" }
                    li class="mb2" { "⚡ No build step required" }
                }

                h2 class="f3 fw6 white-90 mb3 mt4" { "🔧 Template Rendering" }
                p class="f5 mb3" {
                    "HTML is rendered using "
                    span class="fw6" { "Maud" }
                    ", a compile-time HTML template engine for Rust that ensures:"
                }
                ul class="f5 list pl3 mb4" {
                    li class="mb2" { "✅ Type-safe templates checked at compile time" }
                    li class="mb2" { "🛡️ Automatic escaping to prevent XSS attacks" }
                    li class="mb2" { "⚡ Zero-cost abstraction for HTML generation" }
                    li class="mb2" { "🎯 Rust's powerful type system for your views" }
                }

                h2 class="f3 fw6 white-90 mb3 mt4" { "🚀 Why This Stack?" }
                div class="bg-black-40 br3 pa3" {
                    ul class="f5 list pl3 ma0" {
                        li class="mb2" { "⚡ " span class="fw6" { "Performance" } " - Rust's speed + minimal JavaScript" }
                        li class="mb2" { "🔒 " span class="fw6" { "Safety" } " - Compile-time guarantees prevent bugs" }
                        li class="mb2" { "🎯 " span class="fw6" { "Simplicity" } " - Less code to maintain and debug" }
                        li class="mb2" { "📦 " span class="fw6" { "Small bundle" } " - Faster load times for users" }
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
