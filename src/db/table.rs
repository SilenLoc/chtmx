use crate::config;

pub type Ch = clickhouse::Client;

#[derive(Debug, Clone)]
pub enum ColumnData {
    String(Vec<String>),
    Int64(Vec<i64>),
    Float64(Vec<f64>),
    Boolean(Vec<bool>),
    Date(Vec<String>),     // Store as string in YYYY-MM-DD format
    DateTime(Vec<String>), // Store as string in datetime format
}

impl ColumnData {
    /// Get the ClickHouse type name for this column data
    pub fn clickhouse_type(&self) -> &str {
        match self {
            ColumnData::String(_) => "String",
            ColumnData::Int64(_) => "Int64",
            ColumnData::Float64(_) => "Float64",
            ColumnData::Boolean(_) => "Bool",
            ColumnData::Date(_) => "Date",
            ColumnData::DateTime(_) => "DateTime",
        }
    }

    /// Get the number of values in this column
    pub fn len(&self) -> usize {
        match self {
            ColumnData::String(v) => v.len(),
            ColumnData::Int64(v) => v.len(),
            ColumnData::Float64(v) => v.len(),
            ColumnData::Boolean(v) => v.len(),
            ColumnData::Date(v) => v.len(),
            ColumnData::DateTime(v) => v.len(),
        }
    }

    /// Format a value at the given index for SQL insertion
    pub fn format_value(&self, idx: usize) -> String {
        match self {
            ColumnData::String(vals) => {
                format!(
                    "'{}'",
                    vals.get(idx).unwrap_or(&String::new()).replace('\'', "\\'")
                )
            }
            ColumnData::Int64(vals) => vals.get(idx).unwrap_or(&0).to_string(),
            ColumnData::Float64(vals) => vals.get(idx).unwrap_or(&0.0).to_string(),
            ColumnData::Boolean(vals) => {
                if *vals.get(idx).unwrap_or(&false) {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            ColumnData::Date(vals) => {
                format!("'{}'", vals.get(idx).unwrap_or(&String::new()))
            }
            ColumnData::DateTime(vals) => {
                format!("'{}'", vals.get(idx).unwrap_or(&String::new()))
            }
        }
    }

    /// Get a value at the given index as a string for display
    pub fn get_value_as_string(&self, idx: usize) -> String {
        match self {
            ColumnData::String(vals) => vals.get(idx).cloned().unwrap_or_default(),
            ColumnData::Int64(vals) => vals.get(idx).map(|v| v.to_string()).unwrap_or_default(),
            ColumnData::Float64(vals) => vals.get(idx).map(|v| v.to_string()).unwrap_or_default(),
            ColumnData::Boolean(vals) => vals.get(idx).map(|v| v.to_string()).unwrap_or_default(),
            ColumnData::Date(vals) => vals.get(idx).cloned().unwrap_or_default(),
            ColumnData::DateTime(vals) => vals.get(idx).cloned().unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data: ColumnData,
}

impl Column {
    pub fn new(name: String, data: ColumnData) -> Self {
        Self { name, data }
    }

    /// Get the ClickHouse type name for this column
    pub fn clickhouse_type(&self) -> &str {
        self.data.clickhouse_type()
    }

    /// Get the number of values in this column
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Format a value at the given index for SQL insertion
    pub fn format_value(&self, idx: usize) -> String {
        self.data.format_value(idx)
    }

    /// Get a value at the given index as a string for display
    pub fn get_value_as_string(&self, idx: usize) -> String {
        self.data.get_value_as_string(idx)
    }
}

pub struct DynTable {
    pub name: String,
    pub columns: Vec<Column>,
}

impl DynTable {
    pub fn new(name: String, columns: Vec<Column>) -> Self {
        Self { name, columns }
    }

    /// Get the number of rows in the table
    pub fn row_count(&self) -> usize {
        self.columns.first().map(|c| c.len()).unwrap_or(0)
    }

    /// Get the number of columns in the table
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Get a value at a specific row and column index as a string for display
    pub fn get_value_as_string(&self, row_idx: usize, col_idx: usize) -> String {
        self.columns
            .get(col_idx)
            .map(|col| col.get_value_as_string(row_idx))
            .unwrap_or_default()
    }
}

/// Infer and convert a column of string values to the appropriate typed ColumnData
fn infer_and_convert_column_data(values: &[String]) -> ColumnData {
    if values.is_empty() {
        return ColumnData::String(Vec::new());
    }

    // Try to parse as boolean
    let bool_result: Result<Vec<bool>, _> = values
        .iter()
        .map(|v| {
            let trimmed = v.trim().to_lowercase();
            if trimmed == "true" || trimmed == "1" {
                Ok(true)
            } else if trimmed == "false" || trimmed == "0" || trimmed.is_empty() {
                Ok(false)
            } else {
                Err(())
            }
        })
        .collect();

    if let Ok(bool_vals) = bool_result {
        return ColumnData::Boolean(bool_vals);
    }

    // Try to parse as integer
    let int_result: Result<Vec<i64>, _> = values
        .iter()
        .map(|v| {
            let trimmed = v.trim();
            if trimmed.is_empty() {
                Ok(0) // Treat empty as 0
            } else {
                trimmed.parse::<i64>().map_err(|_| ())
            }
        })
        .collect();

    if let Ok(int_vals) = int_result {
        return ColumnData::Int64(int_vals);
    }

    // Try to parse as float
    let float_result: Result<Vec<f64>, _> = values
        .iter()
        .map(|v| {
            let trimmed = v.trim();
            if trimmed.is_empty() {
                Ok(0.0) // Treat empty as 0.0
            } else {
                trimmed.parse::<f64>().map_err(|_| ())
            }
        })
        .collect();

    if let Ok(float_vals) = float_result {
        return ColumnData::Float64(float_vals);
    }

    // Try to parse as date (YYYY-MM-DD)
    let date_result: Result<Vec<String>, _> = values
        .iter()
        .map(|v| {
            let trimmed = v.trim();
            if trimmed.is_empty() {
                Ok(String::new())
            } else if chrono::NaiveDate::parse_from_str(trimmed, "%Y-%m-%d").is_ok() {
                Ok(trimmed.to_string())
            } else {
                Err(())
            }
        })
        .collect();

    if let Ok(date_vals) = date_result {
        return ColumnData::Date(date_vals);
    }

    // Try to parse as datetime
    let datetime_formats = [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M:%S%.f",
        "%Y-%m-%dT%H:%M:%S%.f",
    ];

    let datetime_result: Result<Vec<String>, _> = values
        .iter()
        .map(|v| {
            let trimmed = v.trim();
            if trimmed.is_empty() {
                Ok(String::new())
            } else {
                let is_valid = datetime_formats
                    .iter()
                    .any(|fmt| chrono::NaiveDateTime::parse_from_str(trimmed, fmt).is_ok());
                if is_valid {
                    Ok(trimmed.to_string())
                } else {
                    Err(())
                }
            }
        })
        .collect();

    if let Ok(datetime_vals) = datetime_result {
        return ColumnData::DateTime(datetime_vals);
    }

    // Default to string
    ColumnData::String(values.to_vec())
}

/// Convert row-based data to column-based typed data
pub(super) fn convert_rows_to_typed_columns(
    headers: &[String],
    rows: &[Vec<String>],
) -> Vec<Column> {
    headers
        .iter()
        .enumerate()
        .map(|(col_idx, header)| {
            let column_values: Vec<String> = rows
                .iter()
                .filter_map(|row| row.get(col_idx).cloned())
                .collect();
            let data = infer_and_convert_column_data(&column_values);
            Column::new(header.clone(), data)
        })
        .collect()
}

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

    // Convert rows to typed columns
    let columns = convert_rows_to_typed_columns(&headers, &rows);

    let table = DynTable::new(csv_file_name.to_string(), columns);
    create_dyn_table(table, ch).await?;
    Ok(())
}

pub async fn create_dyn_table(table: DynTable, ch: Ch) -> Result<(), Box<dyn std::error::Error>> {
    // Build column definitions with inferred types
    let columns_with_types = table
        .columns
        .iter()
        .map(|col| format!("`{}` {}", col.name, col.clickhouse_type()))
        .collect::<Vec<_>>()
        .join(", ");

    ch.query(&format!(
        "CREATE TABLE IF NOT EXISTS {} ({}) ENGINE = MergeTree() ORDER BY tuple()",
        table.name, columns_with_types
    ))
    .execute()
    .await?;

    // Insert rows
    let row_count = table.row_count();
    if row_count > 0 {
        let values_list: Vec<String> = (0..row_count)
            .map(|row_idx| {
                let row_values: Vec<String> = table
                    .columns
                    .iter()
                    .map(|col| col.format_value(row_idx))
                    .collect();
                format!("({})", row_values.join(", "))
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

pub async fn get_dyn_table(
    config: &config::Server,
    database: &str,
    table: &str,
    limit: usize,
    offset: usize,
    filters: &std::collections::HashMap<String, Vec<String>>,
) -> Result<DynTable, Box<dyn std::error::Error>> {
    // Build WHERE clause from filters
    let where_clause = if filters.is_empty() {
        String::new()
    } else {
        let conditions: Vec<String> = filters
            .iter()
            .map(|(col, values)| {
                if values.len() == 1 {
                    // Single value: use exact match
                    let escaped_val = values[0].replace('\'', "\'\'");
                    format!("`{}` = '{}'", col, escaped_val)
                } else {
                    // Multiple values: use IN clause
                    let escaped_values: Vec<String> = values
                        .iter()
                        .map(|v| format!("'{}'", v.replace('\'', "\'\'")))
                        .collect();
                    format!("`{}` IN ({})", col, escaped_values.join(", "))
                }
            })
            .collect();
        format!(" WHERE {}", conditions.join(" AND "))
    };

    // Query ClickHouse with TSVWithNames format (tab-separated with header row)
    let query = format!(
        "SELECT * FROM {}.{}{} LIMIT {} OFFSET {} FORMAT TSVWithNames",
        database, table, where_clause, limit, offset
    );

    // Build the HTTP client
    let client = reqwest::Client::new();

    // Build the request
    let mut request = client
        .post(config.clickhouse_url())
        .query(&[("user", config.clickhouse_user())])
        .body(query);

    // Add password if present
    if !config.clickhouse_password().is_empty() {
        request = request.query(&[("password", config.clickhouse_password())]);
    }

    let response = request.send().await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("ClickHouse error: {}", error_text).into());
    }

    let tsv_data = response.text().await?;

    let mut lines = tsv_data.lines();

    let headers_line = lines.next().ok_or("Empty response from ClickHouse")?;
    let fields: Vec<String> = headers_line.split('\t').map(|s| s.to_string()).collect();

    // Remaining lines are data rows
    let rows: Vec<Vec<String>> = lines
        .map(|line| line.split('\t').map(|s| s.to_string()).collect())
        .collect();

    // Convert rows to typed columns
    let columns = convert_rows_to_typed_columns(&fields, &rows);

    Ok(DynTable::new(table.to_string(), columns))
}

/// Get distinct values for a specific column
#[allow(clippy::too_many_arguments)]
pub async fn get_distinct_column_values(
    config: &config::Server,
    database: &str,
    table: &str,
    column: &str,
    limit: usize,
    offset: usize,
    search: Option<&str>,
    filters: &std::collections::HashMap<String, Vec<String>>,
) -> Result<DynTable, Box<dyn std::error::Error>> {
    // Build WHERE clause from filters (excluding current column) and search
    let mut conditions: Vec<String> = Vec::new();

    // Add filters from other columns
    for (col, values) in filters {
        if col != column {
            // Only apply filters from other columns
            if values.len() == 1 {
                // Single value: use exact match
                let escaped_val = values[0].replace('\'', "\'\'");
                conditions.push(format!("`{}` = '{}'", col, escaped_val));
            } else if !values.is_empty() {
                // Multiple values: use IN clause
                let escaped_values: Vec<String> = values
                    .iter()
                    .map(|v| format!("'{}'", v.replace('\'', "\'\'")))
                    .collect();
                conditions.push(format!("`{}` IN ({})", col, escaped_values.join(", ")));
            }
        }
    }

    // Add search filter if provided
    if let Some(search_term) = search
        && !search_term.is_empty()
    {
        let escaped = search_term.replace('\'', "\'\'");
        conditions.push(format!("`{}` ILIKE '%{}%'", column, escaped));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!(" WHERE {}", conditions.join(" AND "))
    };

    // Query for distinct values
    let query = format!(
        "SELECT DISTINCT `{}` FROM {}.{}{} ORDER BY `{}` LIMIT {} OFFSET {} FORMAT TSVWithNames",
        column, database, table, where_clause, column, limit, offset
    );

    // Build the HTTP client
    let client = reqwest::Client::new();

    // Build the request
    let mut request = client
        .post(config.clickhouse_url())
        .query(&[("user", config.clickhouse_user())])
        .body(query);

    // Add password if present
    if !config.clickhouse_password().is_empty() {
        request = request.query(&[("password", config.clickhouse_password())]);
    }

    let response = request.send().await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("ClickHouse error: {}", error_text).into());
    }

    let tsv_data = response.text().await?;

    let mut lines = tsv_data.lines();

    let headers_line = lines.next().ok_or("Empty response from ClickHouse")?;
    let fields: Vec<String> = headers_line.split('\t').map(|s| s.to_string()).collect();

    // Remaining lines are data rows
    let rows: Vec<Vec<String>> = lines
        .map(|line| line.split('\t').map(|s| s.to_string()).collect())
        .collect();

    // Convert rows to typed columns
    let columns = convert_rows_to_typed_columns(&fields, &rows);

    Ok(DynTable::new(column.to_string(), columns))
}
