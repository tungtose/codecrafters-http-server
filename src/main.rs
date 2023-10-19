use std::collections::HashMap;
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

    let http_request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("HTTP: {:#?}", http_request);

    let request_line = http_request.get(0).unwrap();

    let mut headers_map: HashMap<String, String> = HashMap::new();

    for line in http_request.iter() {
        if line.contains(':') {
            let splited = line.split(':').collect::<Vec<&str>>();
            let key = splited.first().unwrap().trim().to_string();
            let val = splited.get(1).unwrap().trim().to_string();
            headers_map.insert(key, val);
        }
    }

    println!("Request line: {:#?}", request_line);
    println!("Headers: {:#?}", headers_map);

    let echo_request = "/echo/";
    let user_agent_request = "/user-agent";
    let ok_status = "HTTP/1.1 200 OK\r\n\r\n";
    let not_found_status = "HTTP/1.1 404 Not Found\r\n\r\n";

    if request_line == "GET / HTTP/1.1" {
        writer.write_all(ok_status.as_bytes()).unwrap();
    } else if request_line.contains(echo_request) {
        let splited_req = request_line.split(' ').collect::<Vec<&str>>();

        let path = splited_req.get(1).unwrap();
        let random_string = path
            .split(echo_request)
            .collect::<Vec<&str>>()
            .pop()
            .unwrap();

        let content_type = "Content-Type: text/plain";

        let content_length = format!("Content-Length: {}", random_string.len());

        let response =
            format!("HTTP/1.1 200 OK\r\n{content_type}\r\n{content_length}\r\n\r\n{random_string}");

        writer.write_all(response.as_bytes()).unwrap();
    } else if request_line.contains(user_agent_request) {
        let user_agent_header = headers_map.get("User-Agent").unwrap();

        let content_type = "Content-Type: text/plain";

        let content_length = format!("Content-Length: {}", user_agent_header.len());

        let response = format!(
            "HTTP/1.1 200 OK\r\n{content_type}\r\n{content_length}\r\n\r\n{user_agent_header}"
        );

        writer.write_all(response.as_bytes()).unwrap();
    } else {
        writer.write_all(not_found_status.as_bytes()).unwrap();
    }
}
