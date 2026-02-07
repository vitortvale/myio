use std::net::TcpListener;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3575")?;
    println!("Hello from myio!");

    loop {
        let (mut stream, _) = listener.accept()?;
        let mut buf = [0; 128];

        let _ = stream.read(&mut buf)?;
        stream.write(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nok")?;
    }
}
