use urlencoding::decode;

use crate::{
    http::{DisplayHeaders, HttpMethod, HttpVersion},
    utils::remove_trailing_slash,
};
use std::collections::HashMap;

pub struct HttpRequest {
    method: HttpMethod,
    uri_raw: String,
    uri: String,
    version: HttpVersion,
    headers: HashMap<String, String>,
    get_params: HashMap<String, Vec<String>>,
    body: String,
}

impl HttpRequest {
    pub fn parse(http_request: String) -> Result<Self, ()> {
        let (first_line, headers_and_body) =
            http_request.split_at(http_request.find('\r').unwrap());

        let first_line_vec: Vec<&str> = first_line.split(' ').collect();
        let method = HttpMethod::from(String::from(first_line_vec[0])).unwrap();
        let mut uri = String::from(first_line_vec[1]);
        let uri_raw = uri.clone();
        let mut get_params: HashMap<String, Vec<String>> = HashMap::new();

        let query_string = if let Some(i) = uri.find('?') {
            let query = uri[i + 1..].to_string();
            uri.truncate(i);
            query
        } else {
            String::new()
        };

        for pair in query_string.split('&') {
            if pair.trim().is_empty() {
                continue;
            }

            let mut split = pair.splitn(2, '=');
            let raw_key = split.next().ok_or(())?;
            let raw_val = split.next();

            if raw_key.is_empty() {
                return Err(());
            }

            let key = decode(raw_key).map_err(|_| ())?.into_owned();
            let val = match raw_val {
                Some(v) => decode(v).map_err(|_| ())?.into_owned(),
                None => String::new(),
            };

            get_params.entry(key).or_default().push(val)
        }

        let version = HttpVersion::from(String::from(first_line_vec[2])).unwrap();

        let mut headers = HashMap::new();

        let (headers_str, body) =
            headers_and_body.split_at(headers_and_body.find("\r\n\r\n").unwrap());

        let _ = headers_str.split("\r\n").collect::<Vec<&str>>()[1..]
            .iter()
            .map(|e| e.to_owned())
            .collect::<Vec<&str>>()
            .iter()
            .map(|e| String::from(e.to_owned()))
            .collect::<Vec<String>>()
            .iter()
            .map(|e| e.split(": ").collect())
            .collect::<Vec<Vec<&str>>>()
            .iter()
            .map(|e| {
                headers.insert(String::from(e[0]), String::from(e[1]));
            })
            .collect::<Vec<()>>();

        Ok(HttpRequest {
            method,
            uri_raw,
            uri: remove_trailing_slash(uri),
            version,
            headers,
            get_params,
            body: String::from(body),
        })
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn version(&self) -> &HttpVersion {
        &self.version
    }

    pub fn get_params(&self) -> &HashMap<String, Vec<String>> {
        &self.get_params
    }

    pub fn uri_raw(&self) -> &str {
        &self.uri_raw
    }
}

impl std::fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header_text = self.headers().clone().to_headers_string();

        write!(
            f,
            "{} {} {}\r\n{}\r\n\r\n{}",
            self.method,
            self.uri_raw,
            self.version,
            header_text,
            self.body()
        )
    }
}
