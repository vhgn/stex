use dotenvy::dotenv;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let db = libsql_client::Client::from_env().await?;

    let q = db.execute("select * from users").await?;
    dbg!(q.rows);

    Ok(())
}
