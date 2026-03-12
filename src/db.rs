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

#[allow(unused)]
pub async fn create_dyn_table(table: DynTable, ch: Ch) -> DynTable {
    ch.query(&format!(
        "CREATE TABLE IF NOT EXISTS {} ({}) ENGINE = MergeTree() ORDER BY tuple()",
        table.name,
        table.fields.join(", ")
    ))
    .execute()
    .await
    .unwrap();

    // insert rows
    if !table.rows.is_empty() {
        let values_list: Vec<String> = table
            .rows
            .iter()
            .map(|row| format!("({})", row.join(", ")))
            .collect();

        ch.query(&format!(
            "INSERT INTO {} VALUES {}",
            table.name,
            values_list.join(", ")
        ))
        .execute()
        .await
        .unwrap();
    }

    table
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
