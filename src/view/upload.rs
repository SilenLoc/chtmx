use actix_web::Result as AwResult;
use actix_web::{HttpRequest, get};
use maud::html;

#[get("/upload")]
pub async fn upload_page(req: HttpRequest) -> AwResult<maud::Markup> {
    let content = html! {
        div class="mw6 center" {
            h1 class="f2 fw2 white-90 mb0 lh-title tc" { "Upload CSV to ClickHouse" }
            p class="f5 white-80 mt3 mb4 tc" {
                "Upload a CSV file to create a new table in ClickHouse"
            }

            div class="bg-white-10 pa4 br3 mt4" {
                form
                    id="upload-form"
                    hx-post="/upload/csv"
                    hx-encoding="multipart/form-data"
                    hx-target="#upload-result"
                    hx-swap="innerHTML"
                    class="w-100" {

                    div class="mb3" {
                        label class="db fw6 lh-copy f5 mb2 white-90" for="csv-file" {
                            "Select CSV File"
                        }
                        input
                            class="input-reset ba b--white-20 pa3 w-100 br2 f5 bg-white-10 white"
                            type="file"
                            id="csv-file"
                            name="file"
                            accept=".csv"
                            required;
                    }

                    div class="mb3" {
                        p class="f6 white-70 lh-copy ma0" {
                            "The table will be created with the same name as your file. "
                            "All columns will be automatically detected from the CSV headers."
                        }
                    }

                    button
                        class="button-reset bn bg-blue white br2 pa3 w-100 f5 pointer grow"
                        type="submit" {
                        "Upload and Create Table"
                    }
                }

                div id="upload-result" class="mt4" {
                    // Results will be shown here
                }
            }

            div class="mt5 bt b--white-20 pt4" {
                h2 class="f4 fw6 white-90 mb3" { "Instructions" }
                ul class="f6 white-80 lh-copy list pl0" {
                    li class="mb2 flex items-start" {
                        span class="mr2" { "•" }
                        span { "Prepare a CSV file with headers in the first row" }
                    }
                    li class="mb2 flex items-start" {
                        span class="mr2" { "•" }
                        span { "The file name (without .csv) will become the table name" }
                    }
                    li class="mb2 flex items-start" {
                        span class="mr2" { "•" }
                        span { "All columns will be stored as String type" }
                    }
                    li class="mb2 flex items-start" {
                        span class="mr2" { "•" }
                        span { "The table will use MergeTree engine" }
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
