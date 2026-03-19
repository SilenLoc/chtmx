use clickhouse::Row;
use serde::{Deserialize, Serialize};

use super::table::Ch;

#[derive(Serialize, Deserialize, Row)]
pub struct Database {
    pub name: String,
}

pub async fn all_databases(ch: Ch) -> Vec<Database> {
    let Ok(mut cursor) = ch
        .query("SELECT name FROM system.databases")
        .fetch::<Database>()
    else {
        log::error!("Failed to fetch databases");
        return Vec::new();
    };
    let mut databases = Vec::new();
    while let Ok(Some(row)) = cursor.next().await {
        databases.push(row);
    }
    databases
}

#[derive(Serialize, Deserialize, Row)]
pub struct Table {
    pub name: String,
}

pub async fn all_tables(ch: Ch, database: &str) -> Vec<Table> {
    let Ok(mut cursor) = ch
        .query("SELECT name FROM system.tables WHERE database = ?")
        .bind(database)
        .fetch::<Table>()
    else {
        log::error!("Failed to fetch tables for database: {}", database);
        return Vec::new();
    };
    let mut tables = Vec::new();
    while let Ok(Some(row)) = cursor.next().await {
        tables.push(row);
    }
    tables
}
