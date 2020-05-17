use sqlx::{SqliteConnection, Connect, Connection};
use anyhow::{Result};

#[async_std::main]
async fn main() -> Result<()>{
    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;

    sqlx::query(
    "create table entries(
        id INTEGER NOT NULL,
        start TEXT,
        stop TEXT,
        week_day TEXT,
        code TEXT,
        memo TEXT)"
    ).execute(&mut conn).await?;

    conn.close();

    Ok(())
}