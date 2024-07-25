#![allow(dead_code)]

use std::{collections::HashMap, fmt::Display};

use parts::{HttpMethod, StatusCode};

pub mod parts;

pub struct Response<'a> {
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
    body: String,
    status_code: StatusCode,
    content_type: Option<&'a str>,
}

#[derive(Debug)]
pub struct Request<T> {
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
    body: Option<T>,
    method: HttpMethod,
    version: String,
    uri: String,
}

impl<'a> Display for Response<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let headers = self
            .headers
            .clone()
            .into_keys()
            .map(|k| format!("\r\n{}: {}", k.clone(), self.headers.get(&k).unwrap()))
            .reduce(|mut k, i| {
                k.push_str(i.as_str());
                return k;
            })
            .unwrap_or_else(|| String::from(""));

        let cookies = self
            .cookies
            .clone()
            .into_keys()
            .map(|k| format!("{}={};", k.clone(), self.cookies.get(&k).unwrap()))
            .reduce(|mut k, i| {
                k.push_str(i.as_str());
                return k;
            })
            .unwrap_or_else(|| String::from(""));

        let response: String = format!(
            "{} {}\r\nContent-Type: {}\r\nDate: {}\r\nConnection: Keep-Alive\r\nAge: 0\r\nContent-Length: {}\r\nSet-Cookie:{}{}\r\n\r\n{}",
            dotenvy::var("HTTP_VERSION").unwrap_or_else(|_| "HTTP/1.1".to_string()),
            self.status_code.to_string(),
            self.content_type.as_ref().unwrap_or_else(|| &"text/html"),
            chrono::Utc::now().to_rfc2822(),
            self.body.len(),
            cookies,
            headers,
            self.body
        );

        write!(f, "{}", response)
    }
}

impl<'a> Response<'a> {
    pub fn new() -> Self {
        return Response {
            headers: HashMap::new(),
            cookies: HashMap::new(),
            body: String::from(""),
            status_code: StatusCode::OK,
            content_type: None,
        };
    }

    pub fn set_status(mut self, status: StatusCode) -> Self {
        self.status_code = status;
        self
    }

    pub fn set_header(mut self, k: String, v: String) -> Self {
        self.headers.insert(k, v);
        self
    }
    pub fn has_header(self, k: String) -> bool {
        self.headers.contains_key(&k)
    }
    pub fn remove_header(mut self, k: String) -> Self {
        self.headers.remove(&k);
        self
    }

    pub fn set_cookie(mut self, k: String, v: String) -> Self {
        self.cookies.insert(k, v);
        self
    }
    pub fn has_cookie(self, k: String) -> bool {
        self.cookies.contains_key(&k)
    }
    pub fn remove_cookie(mut self, k: String) -> Self {
        self.cookies.remove(&k);
        self
    }

    pub fn set_body(mut self, k: String) -> Self {
        self.body = k;
        self
    }
    pub fn content_length(&self) -> usize {
        self.body.len()
    }
}

impl<T> Display for Request<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let headers = self
            .headers
            .clone()
            .into_keys()
            .map(|k| format!("\r\n{}: {}", k.clone(), self.headers.get(&k).unwrap()))
            .reduce(|mut k, i| {
                k.push_str(i.as_str());
                return k;
            })
            .unwrap_or_else(|| String::from(""));

        // let cookies = self
        //     .cookies
        //     .clone()
        //     .into_keys()
        //     .map(|k| format!("{}={};", k.clone(), self.cookies.get(&k).unwrap()))
        //     .reduce(|mut k, i| {
        //         k.push_str(i.as_str());
        //         return k;
        //     })
        //     .unwrap_or_else(|| String::from(""));

        let response: String = format!(
            "{} {} {}{}\r\n\r\n",
            self.method.to_string(),
            self.uri,
            self.version,
            headers,
        );

        write!(f, "{response}")
    }
}

impl<T> Request<T> {
    pub fn new(req: String) -> Self {
        let mut lines = req.lines();
        let request_line = lines.next().unwrap_or_else(|| "");
        let mut request_line_parts = request_line.split_whitespace();

        let method = request_line_parts
            .next()
            .unwrap_or_else(|| "GET")
            .to_string();
        let path = request_line_parts.next().unwrap_or_else(|| "/").to_string();
        let version = request_line_parts
            .next()
            .unwrap_or_else(|| "HTTP/1.1")
            .to_string();

        let mut headers = HashMap::new();
        for line in lines {
            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        }

        Self {
            method: HttpMethod::from_str(&method),
            uri: path,
            version,
            headers,
            cookies: HashMap::new(),
            body: None,
        }
    }

    pub fn new_from_part(
        method: HttpMethod,
        version: String,
        uri: String,
        headers: HashMap<String, String>,
        cookies: HashMap<String, String>,
        body: Option<T>,
    ) -> Self {
        Request {
            headers,
            cookies,
            body,
            method,
            version,
            uri,
        }
    }
}
