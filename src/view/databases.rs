use actix_web::Result as AwResult;
use actix_web::{HttpRequest, get, web};
use maud::html;

use crate::db;

#[get("/databases")]
pub async fn databases_page(req: HttpRequest, ch: web::Data<db::Ch>) -> AwResult<maud::Markup> {
    let databases = db::all_databases(ch.get_ref().clone()).await;

    let content = html! {
        div class="w-100" {
            // Control area at the very top - full width with horizontal layout
            div class="bg-black-70 pa3" {
                div class="mw8 center" {
                    div class="flex items-end" {
                        // Database dropdown
                        div class="mr3 flex" style="min-width: 250px;" {
                            label class="db fw6 lh-copy f6 mr2 white-90" for="database-select" {
                                "Database"
                            }
                            select
                                id="database-select"
                                name="database"
                                class="input-reset ba b--white-30 pa2 w-100 br2 f6 bg-white-10 white"
                                style="color: white;"
                                hx-get="/databases/tables"
                                hx-include="[name='database']"
                                hx-target="#table-select-container"
                                hx-swap="innerHTML"
                                hx-trigger="change" {
                                option value="" selected disabled style="background-color: #1a1a1a; color: #ccc;" { "Choose a database..." }
                                @for database in &databases {
                                    option value=(database.name) style="background-color: #1a1a1a; color: white;" { (database.name) }
                                }
                            }
                        }

                        // Table dropdown container (will be populated via HTMX)
                        div id="table-select-container" style="min-width: 250px;" {
                            // Tables dropdown will be loaded here
                        }
                    }
                }
            }

            // Main content area
            div class="mw8 center ph3 mt4" {
                h1 class="f2 fw2 white-90 mb3 lh-title" { "Database Details" }

                // Placeholder for future functionality
                div class="bg-white-10 pa4 br3 mt4" {
                    p class="white-70 f6 i tc" {
                        "Select a database and table to view details"
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

#[get("/databases/tables")]
pub async fn get_tables(
    database: web::Query<std::collections::HashMap<String, String>>,
    ch: web::Data<db::Ch>,
) -> AwResult<maud::Markup> {
    let db_name = database.get("database").map(|s| s.as_str()).unwrap_or("");

    if db_name.is_empty() {
        return Ok(html! {});
    }

    let tables = db::all_tables(ch.get_ref().clone(), db_name).await;

    Ok(html! {

     div class="mr3 flex" style="min-width: 250px;" {
        label class="db fw6 lh-copy f6 mr2 white-90" for="table-select" {
            "Table"
        }
        select
            id="table-select"
            name="table"
            class="input-reset ba b--white-30 pa2 w-100 br2 f6 bg-white-10 white"
            style="color: white;" {
            option value="" selected disabled style="background-color: #1a1a1a; color: #ccc;" { "Choose a table..." }
            @for table in &tables {
                option value=(table.name) style="background-color: #1a1a1a; color: white;" { (table.name) }
            }
        }
        }
    })
}
