use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> std::io::Result<()> {
    println!("Server is running!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(_e) => {
                println!("Connection failed");
            }
        }
    }

    Ok(())
}

fn handle_connection(stream: TcpStream) {
    let stream_clone = stream.try_clone().unwrap();

    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream_clone);

    loop {
        let mut s = String::new();

        reader.read_line(&mut s).unwrap();

        writer
            .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
            .unwrap();

        writer.flush().unwrap();
    }
}
