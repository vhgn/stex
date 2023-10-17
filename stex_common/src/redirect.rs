use std::io;
use std::net::TcpStream;
use std::thread;

pub fn redirect_stream(mut incoming_stream: TcpStream, host: String) {
    thread::spawn(move || {
        let mut outgoing_stream = TcpStream::connect(host).expect("Failed to connect to host");

        let mut incoming_to_outgoing = incoming_stream
            .try_clone()
            .expect("Failed to clone incoming stream");
        let mut outgoing_to_incoming = outgoing_stream
            .try_clone()
            .expect("Failed to clone outgoing stream");

        thread::spawn(move || {
            io::copy(&mut incoming_to_outgoing, &mut outgoing_stream)
                .expect("Failed to copy stream");
        });

        thread::spawn(move || {
            io::copy(&mut outgoing_to_incoming, &mut incoming_stream)
                .expect("Failed to copy stream");
        });
    });
}
