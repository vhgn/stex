use anyhow::anyhow;
use anyhow::Result;
use dotenvy::dotenv;
use libsql_client::{args, Client, Statement};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let db = Client::from_env().await?;

    let connection = get_domain_connection(db, "exam")
        .await
        .ok_or(anyhow!("domain not found"))?;

    dbg!(connection);

    Ok(())
}

#[derive(Debug)]
struct Connection {
    host: String,
    port: String,
    user: String,
}

async fn get_domain_connection(db: Client, domain: &str) -> Option<Connection> {
    let result = db
        .execute(Statement::with_args(
            "
            select host, port, user
            from domains
            where domain = ?
        ",
            args!(domain),
        ))
        .await
        .ok()?;

    let first = result.rows.first()?;
    let host = first.values.get(0)?;
    let port = first.values.get(1)?;
    let user = first.values.get(2)?;

    Some(Connection {
        host: host.to_string(),
        port: port.to_string().parse().ok()?,
        user: user.to_string(),
    })
}
