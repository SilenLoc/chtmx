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
