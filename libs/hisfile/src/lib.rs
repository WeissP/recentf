mod config;
mod file;
mod tramp;
use file::File;
use sqlx::sqlite::SqliteConnection;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

use anyhow::{Context, Result};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    ConnectOptions,
};

async fn conn() -> Result<SqliteConnection> {
    SqliteConnectOptions::new()
        .filename(config::db_path())
        .connect()
        .await
        .context("")
}

/// Search all matched files.
///
/// # Errors
///
/// This function will return an error if .
pub fn search(
    tramp_id: Option<Vec<usize>>,
    paths: Vec<String>,
    names: Vec<String>,
) -> Result<Vec<File>> {
    todo!()
}

#[tokio::main]
// or #[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    // let pool = SqliteConnection
    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    // let row: (i64,) = sqlx::query_as("SELECT $1")
    //     .bind(150_i64)
    //     .fetch_one(&pool)
    //     .await?;

    // assert_eq!(row.0, 150);

    Ok(())
}
