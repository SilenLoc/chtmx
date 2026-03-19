use actix_web::Result as AwResult;
use actix_web::{HttpRequest, get, web};
use maud::html;

use crate::{config, db};

#[get("/databases")]
pub async fn databases_page(req: HttpRequest, ch: web::Data<db::Ch>) -> AwResult<maud::Markup> {
    let databases = db::all_databases(ch.get_ref().clone()).await;

    let content = html! {
        div class="w-100 flex flex-column" style="height: 100%;" {
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

            // Main content area - flex-auto to fill remaining space
            div class="w-100 ph3 mt4 flex-auto flex flex-column" {
                h1 id="db-heading" class="f4 fw6 white-90 mb3 lh-title" { "Select a database" }

                // Placeholder for future functionality - flex-auto to fill remaining space
                div id="db-content" class="db-content br3 flex-auto flex flex-column" style="overflow: hidden;" {
                    p class="white-70 f6 i tc pa4" {
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
     // Update the heading with the database name
     h1 id="db-heading"
        class="f4 fw6 white-90 mb3 lh-title"
        hx-swap-oob="true" {
        (db_name)
     }

     div class="mr3 flex" style="min-width: 250px;" {
        label class="db fw6 lh-copy f6 mr2 white-90" for="table-select" {
            "Table"
        }
        select
            id="table-select"
            name="table"
            class="input-reset ba b--white-30 pa2 w-100 br2 f6 bg-white-10 white"
            style="color: white;"
            hx-get="/database/tables/table"
            hx-include="[name='database']"
            hx-target="#db-content"
            hx-swap="innerHTML"
            hx-trigger="change"
            {
            option
            value=""
            selected
            disabled
            style="background-color: #1a1a1a; color: #ccc;"
            { "Choose a table..." }
            @for table in &tables {
                option value=(table.name) style="background-color: #1a1a1a; color: white;" { (table.name) }
            }
        }
        }
    })
}

#[get("database/tables/table")]
pub async fn get_table(
    database: web::Query<std::collections::HashMap<String, String>>,
    config: web::Data<config::Server>,
) -> AwResult<maud::Markup> {
    let db_name = database.get("database").map(|s| s.as_str()).unwrap_or("");
    let table = database.get("table").map(|s| s.as_str()).unwrap_or("");

    let table_html = get_table_as_html(&config, db_name, table, 0).await.unwrap();

    // Return both the heading update and the table content
    Ok(html! {
        // Update the heading with database and table name
        h1 id="db-heading"
           class="f4 fw6 white-90 mb3 lh-title"
           hx-swap-oob="true" {
            (db_name) span class="white-50" { " / " } (table)
        }

        // The actual table content
        (table_html)
    })
}

#[get("database/tables/table/rows")]
pub async fn get_table_rows(
    params: web::Query<std::collections::HashMap<String, String>>,
    config: web::Data<config::Server>,
) -> AwResult<maud::Markup> {
    let db_name = params.get("database").map(|s| s.as_str()).unwrap_or("");
    let table = params.get("table").map(|s| s.as_str()).unwrap_or("");
    let offset: usize = params
        .get("offset")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let html = get_table_rows_html(&config, db_name, table, offset)
        .await
        .unwrap();
    Ok(html)
}

pub async fn get_table_as_html(
    config: &config::Server,
    database: &str,
    table: &str,
    offset: usize,
) -> Result<maud::Markup, Box<dyn std::error::Error>> {
    const PAGE_SIZE: usize = 40;

    // Get the table data as DynTable
    let dyn_table = db::get_dyn_table(config, database, table, PAGE_SIZE, offset).await?;

    let next_offset = offset + PAGE_SIZE;
    let has_more_rows = dyn_table.row_count() == PAGE_SIZE;

    // Build styled HTML with Tachyons classes (dark theme with white text)
    let markup = html! {
        div class="overflow-auto flex-auto" style="min-height: 400px;" {
            table class="f6 w-100" cellspacing="0" style="min-width: 800px;" {
                thead {
                    tr {
                        @for column in &dyn_table.columns {
                            th class="fw6 bb b--white-20 tl pv3 pr4 pl3 white-90 bg-dark-orange" style="position: sticky; top: 0; z-index: 10; min-width: 120px;" { (column.name) }
                        }
                    }
                }
                tbody id="table-body" class="lh-copy" {
                    @for row_idx in 0..dyn_table.row_count() {
                        tr class="hover-bg-orange-10" {
                            @for col_idx in 0..dyn_table.column_count() {
                                td class="pv3 pr4 pl3 bb b--white-10 white-80 tl" { (dyn_table.get_value_as_string(row_idx, col_idx)) }
                            }
                        }
                    }
                    @if has_more_rows {
                        tr
                            hx-get={"/database/tables/table/rows?database=" (database) "&table=" (table) "&offset=" (next_offset)}
                            hx-trigger="intersect once"
                            hx-swap="outerHTML" {
                            td colspan="999" class="tc pv3 white-50" {
                                "Loading more..."
                            }
                        }
                    }
                }
            }
        }
    };

    Ok(markup)
}

pub async fn get_table_rows_html(
    config: &config::Server,
    database: &str,
    table: &str,
    offset: usize,
) -> Result<maud::Markup, Box<dyn std::error::Error>> {
    const PAGE_SIZE: usize = 40;

    // Get the table data as DynTable
    let dyn_table = db::get_dyn_table(config, database, table, PAGE_SIZE, offset).await?;

    let next_offset = offset + PAGE_SIZE;
    let has_more_rows = dyn_table.row_count() == PAGE_SIZE;

    // Build just the rows (no table wrapper or header)
    let markup = html! {
        @for row_idx in 0..dyn_table.row_count() {
            tr class="hover-bg-orange-10" {
                @for col_idx in 0..dyn_table.column_count() {
                    td class="pv3 pr4 pl3 bb b--white-10 white-80 tl" { (dyn_table.get_value_as_string(row_idx, col_idx)) }
                }
            }
        }
        @if has_more_rows {
            tr
                hx-get={"/database/tables/table/rows?database=" (database) "&table=" (table) "&offset=" (next_offset)}
                hx-trigger="intersect once"
                hx-swap="outerHTML" {
                td colspan="999" class="tc pv3 white-50" {
                    "Loading more..."
                }
            }
        }
    };

    Ok(markup)
}
