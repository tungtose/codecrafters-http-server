use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> std::io::Result<()> {
    println!("Server is running!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let args = env::args().collect::<Vec<String>>();

                let dir_path: String = if args.get(2).is_some() {
                    args.get(2).unwrap().to_string()
                } else {
                    String::new()
                };

                thread::spawn(move || {
                    handle_connection(stream, &dir_path);
                });
            }
            Err(_e) => {
                println!("Connection failed");
            }
        }
    }

    Ok(())
}

fn get_resource(request_line: &str, spliter: &str) -> String {
    let splited_req = request_line.split(' ').collect::<Vec<&str>>();

    let path = splited_req.get(1).unwrap();
    let resource = path.split(spliter).collect::<Vec<&str>>().pop().unwrap();

    resource.to_string()
}

fn handle_connection(mut stream: TcpStream, dir_path: &str) {
    let stream_clone = stream.try_clone().unwrap();

    let mut reader = BufReader::new(&mut stream);
    let mut writer = BufWriter::new(stream_clone);

    let http_request: Vec<_> = reader
        .by_ref()
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("HTTP: {:#?}", http_request);

    let request_line = http_request.first().unwrap();

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
    let files_request = "/files/";
    let ok_status = "HTTP/1.1 200 OK\r\n\r\n";
    let not_found_status = "HTTP/1.1 404 Not Found\r\n\r\n";

    if request_line.contains("POST") {
        let content_length = headers_map
            .get("Content-Length")
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let mut body = vec![0; content_length as usize];

        reader.read_exact(&mut body).unwrap();

        let file_name = get_resource(request_line, files_request);
        let file_path = format!("{dir_path}/{file_name}");

        fs::write(file_path, body).expect("Unable to write file: {file_name}");

        writer.write_all("HTTP/1.1 201 Created".as_bytes()).unwrap();
    }

    if request_line == "GET / HTTP/1.1" {
        writer.write_all(ok_status.as_bytes()).unwrap();
    } else if request_line.contains(files_request) {
        let file = get_resource(request_line, files_request);
        dbg!(file.clone());

        let file_path = format!("{dir_path}/{file}");
        let contents = fs::read_to_string(file_path);

        match contents {
            Ok(contents) => {
                println!("file:\n {contents}");

                let content_type = "Content-Type: application/octet-stream";

                let content_length = format!("Content-Length: {}", contents.len());

                let response = format!(
                    "HTTP/1.1 200 OK\r\n{content_type}\r\n{content_length}\r\n\r\n{contents}"
                );

                writer.write_all(response.as_bytes()).unwrap();
            }
            Err(_err) => {
                writer.write_all(not_found_status.as_bytes()).unwrap();
            }
        }
    } else if request_line.contains(echo_request) {
        let random_string = get_resource(request_line, echo_request);

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
