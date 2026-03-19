use actix_web::Result as AwResult;
use actix_web::{HttpRequest, get};
use maud::html;

#[get("/how-it-works")]
pub async fn how_it_works_page(req: HttpRequest) -> AwResult<maud::Markup> {
    let content = html! {
        div class="mw7 center" {
            h1 class="f2 f1-l fw2 white-90 mb3 lh-title" { "How It Works" }

            div class="tl white-80 lh-copy" {
                p class="f4 mb4" {
                    "CHTMX demonstrates building fast, interactive web applications using a modern stack: "
                    "Rust for the backend, HTMX for dynamic frontend interactions, and ClickHouse for data storage. "
                    "The entire application runs without heavy JavaScript frameworks or client-side rendering."
                }

                h2 class="f3 fw6 white-90 mb3 mt4" { "Architecture Overview" }
                p class="f5 mb3" {
                    "The application follows a server-side rendering approach where the backend generates HTML fragments "
                    "that are swapped into the page by HTMX. This results in a responsive user experience with minimal "
                    "client-side complexity."
                }

                h2 class="f3 fw6 white-90 mb3 mt4" { "Backend - Rust & Actix-web" }
                p class="f5 mb3" {
                    "The server is built with Actix-web, a high-performance web framework. Each HTTP endpoint returns "
                    "HTML rendered using the Maud template engine. The compile-time template checking ensures type safety "
                    "and prevents XSS vulnerabilities through automatic escaping."
                }
                p class="f5 mb3" {
                    "ClickHouse integration provides fast analytical queries on large datasets. The application uses "
                    "the ClickHouse HTTP interface to execute queries and transform results into HTML tables."
                }

                h2 class="f3 fw6 white-90 mb3 mt4" { "Frontend - HTMX" }
                p class="f5 mb3" {
                    "HTMX enables dynamic content updates through HTML attributes. When a user clicks a navigation link "
                    "or submits a form, HTMX makes an AJAX request and swaps the returned HTML into the specified target element."
                }
                div class="bg-black-40 br3 pa3 mb4" {
                    p class="f6 white-70 ma0 mb2" { "Example: Navigation link with HTMX attributes" }
                    pre class="f7 white-90 ma0 overflow-x-auto" {
                        code { "<a href=\"/databases\" hx-get=\"/databases\" hx-target=\"#feature\">Databases</a>" }
                    }
                }
                p class="f5 mb3" {
                    "This approach eliminates the need for a JavaScript build step while still providing a modern, "
                    "single-page-application feel. Browser history is preserved using hx-push-url."
                }

                h2 class="f3 fw6 white-90 mb3 mt4" { "Styling - Tachyons CSS" }
                p class="f5 mb3" {
                    "Tachyons provides utility-first CSS classes that are composed directly in the HTML. "
                    "The framework is small (14kb), requires no build step, and makes it easy to create consistent, "
                    "responsive designs. Custom color classes extend the base Tachyons palette for project-specific theming."
                }

                h2 class="f3 fw6 white-90 mb3 mt4" { "Data Flow Example" }
                p class="f5 mb3" {
                    "When viewing a database table:"
                }
                ol class="f5 pl3 mb4" {
                    li class="mb2" { "User selects a database from the dropdown (triggers hx-get request)" }
                    li class="mb2" { "Server queries ClickHouse for table list in that database" }
                    li class="mb2" { "Rust renders HTML select element with table options" }
                    li class="mb2" { "HTMX swaps the HTML into the page" }
                    li class="mb2" { "User selects a table (triggers another hx-get request)" }
                    li class="mb2" { "Server queries ClickHouse for table data with pagination" }
                    li class="mb2" { "Rust renders HTML table with the data" }
                    li class="mb2" { "HTMX swaps the table into the content area" }
                    li class="mb2" { "Infinite scroll loads more rows as user scrolls (hx-trigger=\"intersect\")" }
                }

                h2 class="f3 fw6 white-90 mb3 mt4" { "Key Benefits" }
                div class="bg-black-40 br3 pa3" {
                    dl class="f5 ma0" {
                        dt class="fw6 mb1" { "Performance" }
                        dd class="ml0 mb3 white-70" {
                            "Rust provides native-code performance. Minimal JavaScript means faster page loads and lower memory usage."
                        }
                        dt class="fw6 mb1" { "Type Safety" }
                        dd class="ml0 mb3 white-70" {
                            "The Rust compiler catches errors at build time. Maud templates are checked for validity during compilation."
                        }
                        dt class="fw6 mb1" { "Simplicity" }
                        dd class="ml0 mb3 white-70" {
                            "No JavaScript framework complexity. No build toolchain. Server-rendered HTML is easier to reason about and debug."
                        }
                        dt class="fw6 mb1" { "Scalability" }
                        dd class="ml0 mb3 white-70" {
                            "ClickHouse handles analytical workloads efficiently. Actix-web provides async request handling."
                        }
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
