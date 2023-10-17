use std::io::Read;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:6969").expect("Failed to bind to port");

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let mut buffer = String::new();
            stream
                .read_to_string(&mut buffer)
                .expect("Failed to read from stream");
            println!("{}", buffer);
        } else {
            eprintln!("failed to accept connection");
        }
    }
}
