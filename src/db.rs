use clickhouse::Row;
use serde::{Deserialize, Serialize};

use crate::config;

pub type Ch = clickhouse::Client;

pub fn connect(config: &config::Server) -> Ch {
    let mut client = clickhouse::Client::default()
        .with_url(config.clickhouse_url())
        .with_user(config.clickhouse_user());

    if !config.clickhouse_password().is_empty() {
        client = client.with_password(config.clickhouse_password());
    }

    client
}

pub async fn create_table_from_csv(
    ch: Ch,
    csv_file_name: &str,
    csv: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_reader(csv);

    let headers = rdr
        .headers()?
        .iter()
        .map(|s| {
            // Sanitize column names: replace spaces and special chars with underscores
            s.replace(
                [' ', '-', '.', '(', ')', '[', ']', '{', '}', '/', '\\'],
                "_",
            )
        })
        .collect::<Vec<_>>();

    let rows = rdr
        .records()
        .map(|r| r.map(|record| record.iter().map(|s| s.to_string()).collect::<Vec<_>>()))
        .collect::<Result<Vec<_>, _>>()?;

    let table = DynTable::new(csv_file_name.to_string(), headers, rows);
    create_dyn_table(table, ch).await?;
    Ok(())
}

#[allow(unused)]
pub async fn create_dyn_table(table: DynTable, ch: Ch) -> Result<(), Box<dyn std::error::Error>> {
    // Build column definitions with types (all as String)
    let columns_with_types = table
        .fields
        .iter()
        .map(|field| format!("`{}` String", field))
        .collect::<Vec<_>>()
        .join(", ");

    ch.query(&format!(
        "CREATE TABLE IF NOT EXISTS {} ({}) ENGINE = MergeTree() ORDER BY tuple()",
        table.name, columns_with_types
    ))
    .execute()
    .await?;

    // insert rows
    if !table.rows.is_empty() {
        let values_list: Vec<String> = table
            .rows
            .iter()
            .map(|row| {
                let quoted_values = row
                    .iter()
                    .map(|val| {
                        // Escape single quotes in the value and wrap in quotes
                        format!("'{}'", val.replace('\'', "\\'"))
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({})", quoted_values)
            })
            .collect();

        ch.query(&format!(
            "INSERT INTO {} VALUES {}",
            table.name,
            values_list.join(", ")
        ))
        .execute()
        .await?;
    }
    Ok(())
}

#[allow(unused)]
pub struct DynTable {
    pub name: String,
    pub fields: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[allow(unused)]
impl DynTable {
    pub fn new(name: String, fields: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Self { name, fields, rows }
    }
}

#[derive(Serialize, Deserialize, Row)]
pub struct Database {
    pub name: String,
}

pub async fn all_databases(ch: Ch) -> Vec<Database> {
    let mut cursor = ch
        .query("SELECT name FROM system.databases")
        .fetch::<Database>()
        .unwrap();
    let mut databases = Vec::new();
    while let Some(row) = cursor.next().await.unwrap() {
        databases.push(row);
    }
    databases
}

#[derive(Serialize, Deserialize, Row)]
pub struct Table {
    pub name: String,
}

pub async fn all_tables(ch: Ch, database: &str) -> Vec<Table> {
    let mut cursor = ch
        .query("SELECT name FROM system.tables WHERE database = ?")
        .bind(database)
        .fetch::<Table>()
        .unwrap();
    let mut tables = Vec::new();
    while let Some(row) = cursor.next().await.unwrap() {
        tables.push(row);
    }
    tables
}

pub async fn get_table_as_markdown_paginated(
    config: &config::Server,
    database: &str,
    table: &str,
    limit: usize,
    offset: usize,
) -> Result<String, Box<dyn std::error::Error>> {
    let query = format!(
        "SELECT * FROM {}.{} LIMIT {} OFFSET {} FORMAT Markdown",
        database, table, limit, offset
    );
    get_as_markdown(config, &query).await
}

pub async fn get_as_markdown(
    config: &config::Server,
    query: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Ensure the query ends with FORMAT Markdown
    let query_with_format = if query.trim().to_uppercase().contains("FORMAT") {
        query.to_string()
    } else {
        format!("{} FORMAT Markdown", query.trim())
    };

    // Build the HTTP client
    let client = reqwest::Client::new();

    // Build the request
    let mut request = client
        .post(config.clickhouse_url())
        .query(&[("user", config.clickhouse_user())])
        .body(query_with_format);

    // Add password if present
    if !config.clickhouse_password().is_empty() {
        request = request.query(&[("password", config.clickhouse_password())]);
    }

    // Execute the request
    let response = request.send().await?;

    // Check for errors
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("ClickHouse error: {}", error_text).into());
    }

    // Get the response body as text
    let markdown = response.text().await?;
    Ok(markdown)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_get_as_markdown() {
        let config = config::Server::new(
            ("0.0.0.0".to_string(), 8080),
            "info".to_string(),
            "http://localhost:8123".to_string(),
            "default".to_string(),
            String::new(),
        );

        // Test with a simple query
        let result = get_as_markdown(&config, "SELECT number, number * 2 FROM numbers(5)")
            .await
            .expect("Failed to get markdown");

        println!("Markdown result:\n{}", result);

        // Verify it contains markdown table characters
        assert!(result.contains("|"));
        assert!(result.contains("number"));
    }
}
