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

fn handle_connection(mut stream: TcpStream) {
    let stream_clone = stream.try_clone().unwrap();

    let reader = BufReader::new(&mut stream);
    let mut writer = BufWriter::new(stream_clone);

    let request_line = reader.lines().next().unwrap().unwrap();

    println!("Request line: {:#?}", request_line);

    if request_line == "GET / HTTP/1.1" {
        let response = "HTTP/1.1 200 OK\r\n\r\n";

        writer.write_all(response.as_bytes()).unwrap();
    } else {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        writer.write_all(response.as_bytes()).unwrap();
    }
}
