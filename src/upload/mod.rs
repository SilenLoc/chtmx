use actix_multipart::Multipart;
use actix_web::Result as AwResult;
use actix_web::{HttpResponse, post, web};
use futures_util::StreamExt;
use log::{error, info};
use maud::html;

use crate::db;

#[post("/upload/csv")]
pub async fn upload_csv(mut payload: Multipart, ch: web::Data<db::Ch>) -> AwResult<HttpResponse> {
    let mut file_name: Option<String> = None;
    let mut file_data: Vec<u8> = Vec::new();

    // Process multipart form data
    while let Some(field) = payload.next().await {
        let mut field = match field {
            Ok(f) => f,
            Err(e) => {
                error!("Error reading multipart field: {}", e);
                let markup = html! {
                    div class="bg-dark-red white pa3 br2" {
                        p class="f5 fw6 ma0 mb2" { "❌ Upload Error" }
                        p class="f6 ma0" { "Error reading file: " (e) }
                    }
                };
                return Ok(HttpResponse::BadRequest()
                    .content_type("text/html")
                    .body(markup.into_string()));
            }
        };

        let content_disposition = field.content_disposition();
        let field_name = content_disposition
            .and_then(|cd| cd.get_name())
            .unwrap_or("");

        if field_name == "file" {
            // Get filename from content disposition
            if let Some(fname) = content_disposition.and_then(|cd| cd.get_filename()) {
                // Sanitize filename: remove extension and any special characters
                file_name = Some(
                    fname
                        .trim_end_matches(".csv")
                        .replace(['-', ' ', '.'], "_")
                        .to_string(),
                );
            }

            // Read file data
            while let Some(chunk) = field.next().await {
                let chunk = match chunk {
                    Ok(c) => c,
                    Err(e) => {
                        error!("Error reading file chunk: {}", e);
                        let markup = html! {
                            div class="bg-dark-red white pa3 br2" {
                                p class="f5 fw6 ma0 mb2" { "❌ Upload Error" }
                                p class="f6 ma0" { "Error reading file data: " (e) }
                            }
                        };
                        return Ok(HttpResponse::BadRequest()
                            .content_type("text/html")
                            .body(markup.into_string()));
                    }
                };
                file_data.extend_from_slice(&chunk);
            }
        }
    }

    // Validate we received a file
    if file_data.is_empty() {
        let markup = html! {
            div class="bg-orange white pa3 br2" {
                p class="f5 fw6 ma0 mb2" { "⚠️ No File" }
                p class="f6 ma0" { "Please select a CSV file to upload." }
            }
        };
        return Ok(HttpResponse::BadRequest()
            .content_type("text/html")
            .body(markup.into_string()));
    }

    let table_name = file_name.unwrap_or_else(|| "uploaded_table".to_string());

    info!(
        "Processing CSV upload: {} ({} bytes)",
        table_name,
        file_data.len()
    );

    // Create table from CSV
    match db::create_table_from_csv(ch.get_ref().clone(), &table_name, &file_data).await {
        Ok(_) => {
            info!("Successfully created table: {}", table_name);
            let markup = html! {
                div class="bg-dark-green white pa3 br2" {
                    p class="f5 fw6 ma0 mb2" { "✓ Success!" }
                    p class="f6 ma0 mb2" {
                        "Table '" (table_name) "' has been created successfully."
                    }
                    p class="f6 ma0" {
                        "You can now query it using SQL."
                    }
                }
            };
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(markup.into_string()))
        }
        Err(e) => {
            error!("Failed to create table from CSV: {}", e);
            let markup = html! {
                div class="bg-dark-red white pa3 br2" {
                    p class="f5 fw6 ma0 mb2" { "❌ Database Error" }
                    p class="f6 ma0" { "Failed to create table: " (e) }
                }
            };
            Ok(HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(markup.into_string()))
        }
    }
}
