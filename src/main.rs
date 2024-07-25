use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::{TcpListener, TcpStream},
};

use http::{parts::HttpMethod, Request};

mod http;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

// #[derive(Debug)]
// struct HttpRequest {
//     method: String,
//     path: String,
//     http_version: String,
//     headers: HashMap<String, String>,
//     body: HashMap<String, String>,
// }

fn parse_http_request(stream: &mut TcpStream) -> Request<HashMap<String, String>> {
    let mut reader = BufReader::new(stream);
    let mut request_line = String::new();

    // Read the request line
    reader.read_line(&mut request_line).unwrap();
    let mut request_line_parts = request_line.split_whitespace();

    let method = request_line_parts.next().unwrap().to_string();
    let path = request_line_parts.next().unwrap().to_string();
    let http_version = request_line_parts.next().unwrap().to_string();

    let mut headers = HashMap::new();
    let mut body = HashMap::new();
    let mut content_length = 0;

    // Read headers
    for line in reader.by_ref().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
            if key.to_lowercase() == "content-length" {
                content_length = value.parse::<usize>().unwrap();
            }
        }
    }

    // Read body if content length is specified
    if content_length > 0 {
        let mut body_vals = HashMap::new();
        let mut body_vec: Vec<u8> = Vec::new();
        body_vec.resize(content_length, 0);
        reader.read_exact(&mut body_vec).unwrap();
        println!("{}", String::from_utf8(body_vec.clone()).unwrap());
        let body_string = String::from_utf8(body_vec).unwrap();
        let bodybody: Vec<&str> = body_string.rsplit("&").collect();
        let body_arr: Vec<Vec<&str>> = bodybody
            .iter()
            .map(|k| k.split("=").collect::<Vec<_>>())
            .collect();
        for a in body_arr {
            body_vals.insert(a[0].to_string(), a[1].to_string());
        }

        body = body_vals;
    }

    Request::new_from_part(
        HttpMethod::from_str(method.as_str()),
        http_version,
        path,
        headers,
        HashMap::new(),
        Some(body),
    )
}

fn handle_connection(mut stream: TcpStream) {
    let request = parse_http_request(&mut stream);
    println!("{:#?}", request);
    // Handle the request and send a response...
}
