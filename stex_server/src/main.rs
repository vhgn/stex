use anyhow::Result;
use dotenvy::dotenv;

use libsql_client::args;
use libsql_client::Client;
use libsql_client::Statement;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use stex_common::redirect::redirect_stream;
use tokio::runtime::Runtime;

type Db = Arc<Mutex<Client>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let listener = TcpListener::bind("0.0.0.0:80")?;
    let db = Arc::new(Mutex::new(Client::from_env().await?));
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let db = db.clone();
            thread::spawn(move || {
                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    handle_connection(stream, db).await;
                });
            })
            .join()
            .unwrap();
        } else {
            eprintln!("failed to accept connection");
        }
    }

    Ok(())
}

async fn handle_connection(incoming_stream: TcpStream, db: Db) {
    let db = db.lock().unwrap();
    let host = get_host_for_domain(&db, "exam")
        .await
        .expect("Failed to get host");
    println!("host: {}", host);
    redirect_stream(incoming_stream, host)
}

async fn get_host_for_domain(db: &Client, domain: &str) -> Option<String> {
    let result = db
        .execute(Statement::with_args(
            "select host from domains where domain = ?",
            args!(domain),
        ))
        .await
        .ok()?;

    let first = result.rows.first()?;
    let value = first.values.get(0)?;

    Some(value.to_string())
}
