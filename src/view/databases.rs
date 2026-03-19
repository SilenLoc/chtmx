use actix_web::Result as AwResult;
use actix_web::{HttpRequest, get, web};
use maud::{PreEscaped, html};

use crate::{config, db};

#[get("/databases")]
pub async fn databases_page(req: HttpRequest, ch: web::Data<db::Ch>) -> AwResult<maud::Markup> {
    let databases = db::all_databases(ch.get_ref().clone()).await;

    let content = html! {
        div class="w-100 flex flex-column" style="height: 100%;" {
            // Control area at the very top - full width with horizontal layout
            div class="bg-black-70 pa3" {
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
    req: HttpRequest,
    config: web::Data<config::Server>,
) -> AwResult<maud::Markup> {
    // Parse query string manually to get all parameter values (including duplicates)
    let query_str = req.query_string();

    let mut db_name = String::new();
    let mut table_name = String::new();
    let mut filters: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    // Parse query string
    for pair in query_str.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            // Simple URL decode (replace %20 with space, etc)
            let value = value.replace("%20", " ").replace("+", " ");

            match key {
                "database" => db_name = value,
                "table" => table_name = value,
                k if k.starts_with("filter_") => {
                    if !value.is_empty() {
                        let col = k.strip_prefix("filter_").unwrap();
                        filters.entry(col.to_string()).or_default().push(value);
                    }
                }
                _ => {}
            }
        }
    }

    let table_html = get_table_as_html(&config, &db_name, &table_name, 0, &filters)
        .await
        .unwrap();

    // Return both the heading update and the table content
    Ok(html! {
        // Update the heading with database and table name
        h1 id="db-heading"
           class="f4 fw6 white-90 mb3 lh-title"
           hx-swap-oob="true" {
            (db_name) span class="white-50" { " / " } (table_name)
        }

        // The actual table content
        (table_html)
    })
}

#[get("database/tables/table/rows")]
pub async fn get_table_rows(
    req: HttpRequest,
    config: web::Data<config::Server>,
) -> AwResult<maud::Markup> {
    // Parse query string manually to get all parameter values (including duplicates)
    let query_str = req.query_string();

    let mut db_name = String::new();
    let mut table_name = String::new();
    let mut offset = 0;
    let mut filters: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    // Parse query string
    for pair in query_str.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            // Simple URL decode (replace %20 with space, etc)
            let value = value.replace("%20", " ").replace("+", " ");

            match key {
                "database" => db_name = value,
                "table" => table_name = value,
                "offset" => offset = value.parse::<usize>().unwrap_or(0),
                k if k.starts_with("filter_") => {
                    if !value.is_empty() {
                        let col = k.strip_prefix("filter_").unwrap();
                        filters.entry(col.to_string()).or_default().push(value);
                    }
                }
                _ => {}
            }
        }
    }

    let table_html = get_table_rows_html(&config, &db_name, &table_name, offset, &filters)
        .await
        .unwrap();

    Ok(table_html)
}

pub async fn get_table_as_html(
    config: &config::Server,
    database: &str,
    table: &str,
    offset: usize,
    filters: &std::collections::HashMap<String, Vec<String>>,
) -> Result<maud::Markup, Box<dyn std::error::Error>> {
    const PAGE_SIZE: usize = 40;

    // Get the table data as DynTable
    let dyn_table = db::get_dyn_table(config, database, table, PAGE_SIZE, offset, filters).await?;

    let next_offset = offset + PAGE_SIZE;
    let has_more_rows = dyn_table.row_count() == PAGE_SIZE;

    // Build styled HTML with Tachyons classes (dark theme with white text)
    let markup = html! {
        div id="table-container" class="overflow-auto flex-auto relative" style="min-height: 400px;" {
            table class="f6 w-100" cellspacing="0" style="min-width: 800px;" {
                thead {
                    tr {
                        @for column in &dyn_table.columns {
                            th class="fw6 bb b--white-20 pv3 pr4 pl3 white-90 bg-dark-orange relative" style="position: sticky; top: 0; z-index: 10; min-width: 120px;" {
                                div class="flex items-center justify-between" {
                                    span { (column.name) }
                                    // Filter icon for each column
                                    button
                                        class="bn bg-transparent pointer pa1 ml2 filter-trigger"
                                        onclick={"toggleFilter('" (column.name) "', '" (database) "', '" (table) "')"}
                                        title="Filter column"
                                        {
                                        svg class="w1 h1 white-70 hover-white" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" {
                                            path d="M3 3a1 1 0 011-1h12a1 1 0 011 1v3a1 1 0 01-.293.707L12 11.414V15a1 1 0 01-.293.707l-2 2A1 1 0 018 17v-5.586L3.293 6.707A1 1 0 013 6V3z" {}
                                        }
                                    }
                                }
                                // Filter flyout panel (one per column, initially hidden)
                                div id={"filter-flyout-" (column.name)} class="absolute dn bg-near-black ba b--white-20 br2 shadow-3" style="top: 100%; right: 0; width: 280px; max-height: 400px; z-index: 200; margin-top: 0.5rem;" {
                                    // Header
                                    div class="pa3 bb b--white-20 flex items-center justify-between" {
                                        h4 class="f6 fw6 white-90 ma0" { "Filter: " (column.name) }
                                        button
                                            class="bn bg-transparent white-70 hover-white pointer f4 pa0"
                                            onclick={"document.getElementById('filter-flyout-" (column.name) "').classList.add('dn')"}
                                            { "×" }
                                    }
                                    // Search box
                                    div class="pa2 bb b--white-20" {
                                        input
                                            type="text"
                                            id={"search_" (column.name)}
                                            name="search"
                                            placeholder="Search values..."
                                            class="input-reset pa2 w-100 f6 ba b--white-30 br2 white-80 bg-black-20"
                                            hx-get={"/database/tables/table/column/values?database=" (database) "&table=" (table) "&column=" (column.name)
                                                @for (col, vals) in filters {
                                                    @for val in vals {
                                                        "&selected_" (col) "=" (val)
                                                    }
                                                }
                                            }
                                            hx-trigger="keyup changed delay:300ms"
                                            hx-target={"#filter-values-" (column.name)}
                                            hx-include="this"
                                            hx-swap="innerHTML";
                                    }
                                    // Checkbox list container (scrollable)
                                    div id={"filter-values-" (column.name)} class="overflow-y-auto pa2" style="max-height: 250px;"
                                        hx-get={"/database/tables/table/column/values?database=" (database) "&table=" (table) "&column=" (column.name)
                                            @for (col, vals) in filters {
                                                @for val in vals {
                                                    "&selected_" (col) "=" (val)
                                                }
                                            }
                                        }
                                        hx-trigger="load"
                                        hx-swap="innerHTML" {
                                        // Values will be loaded here
                                    }
                                    // Apply button
                                    div class="pa2 bt b--white-20" {
                                        button
                                            class="bn bg-orange white br2 pa2 w-100 pointer hover-bg-dark-orange f6 fw6"
                                            hx-get={"/database/tables/table?database=" (database) "&table=" (table)}
                                            hx-trigger="click"
                                            hx-target="#table-container"
                                            hx-include="[name^='filter_']:checked"
                                            hx-swap="outerHTML"
                                            hx-push-url="true"
                                            { "Apply Filter" }
                                    }
                                }
                            }
                        }
                    }
                }
                tbody id="table-body-container" class="lh-copy" {
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
                            hx-include="[name^='filter_']"
                            hx-swap="outerHTML" {
                            td colspan="999" class="tc pv3 white-50" {
                                "Loading more..."
                            }
                        }
                    }
                }
            }
        }
        // Add JavaScript for toggling filter flyouts
        (PreEscaped(r#"<script>
            function toggleFilter(columnName, database, table) {
                // Close all other flyouts first
                const allFlyouts = document.querySelectorAll('[id^="filter-flyout-"]');
                allFlyouts.forEach(flyout => {
                    if (flyout.id !== 'filter-flyout-' + columnName) {
                        flyout.classList.add('dn');
                    }
                });
                
                // Toggle the current flyout
                const flyout = document.getElementById('filter-flyout-' + columnName);
                if (flyout) {
                    flyout.classList.toggle('dn');
                }
            }
        </script>"#))
    };

    Ok(markup)
}

pub async fn get_table_rows_html(
    config: &config::Server,
    database: &str,
    table: &str,
    offset: usize,
    filters: &std::collections::HashMap<String, Vec<String>>,
) -> Result<maud::Markup, Box<dyn std::error::Error>> {
    const PAGE_SIZE: usize = 40;

    // Get the table data as DynTable
    let dyn_table = db::get_dyn_table(config, database, table, PAGE_SIZE, offset, filters).await?;

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
                hx-include="[name^='filter_']"
                hx-swap="outerHTML" {
                td colspan="999" class="tc pv3 white-50" {
                    "Loading more..."
                }
            }
        }
    };

    Ok(markup)
}

#[get("database/tables/table/column/values")]
pub async fn get_column_values(
    req: HttpRequest,
    config: web::Data<config::Server>,
) -> AwResult<maud::Markup> {
    // Parse query string manually to get all parameter values (including duplicates)
    let query_str = req.query_string();

    let mut db_name = String::new();
    let mut table = String::new();
    let mut column = String::new();
    let mut offset = 0;
    let mut search = None;
    let mut selected_values: Vec<String> = Vec::new();

    // Parse query string
    for pair in query_str.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            // Simple URL decode (replace %20 with space, etc)
            let value = value.replace("%20", " ").replace("+", " ");

            match key {
                "database" => db_name = value,
                "table" => table = value,
                "column" => column = value,
                "offset" => offset = value.parse::<usize>().unwrap_or(0),
                "search" => search = Some(value),
                k if k.starts_with("selected_") => {
                    // Collect currently selected filter values
                    if let Some(col) = k.strip_prefix("selected_")
                        && col == column && !value.is_empty() {
                            selected_values.push(value);
                        }
                }
                _ => {}
            }
        }
    }

    const PAGE_SIZE: usize = 40;

    let dyn_table = db::get_distinct_column_values(
        &config,
        &db_name,
        &table,
        &column,
        PAGE_SIZE,
        offset,
        search.as_deref(),
    )
    .await
    .unwrap();

    let next_offset = offset + PAGE_SIZE;
    let has_more = dyn_table.row_count() == PAGE_SIZE;

    // Build checkbox list for distinct values
    let markup = html! {
        @for row_idx in 0..dyn_table.row_count() {
            @let value = dyn_table.get_value_as_string(row_idx, 0);
            @let is_checked = selected_values.contains(&value);
            label class="db pv2 ph2 hover-bg-white-10 pointer tl" {
                input
                    type="checkbox"
                    name={"filter_" (column)}
                    value={(value)}
                    class="mr2"
                    checked[is_checked];
                span class="white-80" { (value) }
            }
        }
        @if has_more {
            div
                hx-get={"/database/tables/table/column/values?database=" (&db_name) "&table=" (&table) "&column=" (&column) "&offset=" (next_offset)
                    @if let Some(s) = &search { "&search=" (s) }
                    @for val in &selected_values { "&selected_" (column) "=" (val) }
                }
                hx-trigger="intersect once"
                hx-swap="outerHTML"
                class="tc pv2 white-50 f7" {
                "Load more..."
            }
        }
    };

    Ok(markup)
}
