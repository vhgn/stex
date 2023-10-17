use anyhow::Result;
use dotenvy::dotenv;
use futures::executor::block_on;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Pool;
use sqlx::Sqlite;
use stex_common::redirect::redirect_stream;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let listener = TcpListener::bind("0.0.0.0:80")?;
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_connection(stream, pool.clone());
        } else {
            eprintln!("failed to accept connection");
        }
    }

    Ok(())
}

fn handle_connection(incoming_stream: TcpStream, db: Pool<Sqlite>) {
    thread::spawn(move || {
        let host = block_on(get_host_for_domain(&db, "exam")).expect("Failed to get host");
        redirect_stream(incoming_stream, host)
    });
}

#[derive(sqlx::FromRow)]
struct Host {
    host: String,
}

async fn get_host_for_domain(db: &Pool<Sqlite>, domain: &str) -> Option<String> {
    let result: Host = sqlx::query_as("select host from domains where domain = ?")
        .bind(domain)
        .fetch_one(db)
        .await
        .ok()?;

    Some(result.host)
}
