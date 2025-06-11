use crate::http::{DisplayHeaders, HttpStatus, HttpVersion};
use std::{collections::HashMap, fmt::Display};

#[derive(Clone)]
pub struct HttpResponse {
    version: HttpVersion,
    status: HttpStatus,
    headers: HashMap<String, String>,
    body: String,
}
impl HttpResponse {
    pub fn status(&self) -> HttpStatus {
        self.status
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn version(&self) -> &HttpVersion {
        &self.version
    }
}
impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}\r\n{}\r\n\r\n{}",
            self.version,
            self.status,
            self.headers.clone().to_headers_string(),
            self.body
        )
    }
}

pub struct HttpResponseBuilder {
    version: HttpVersion,
    status: HttpStatus,
    headers: HashMap<String, String>,
    body: String,
}
impl HttpResponseBuilder {
    pub fn new() -> Self {
        HttpResponseBuilder {
            version: HttpVersion::Http1_1,
            status: HttpStatus::Ok,
            headers: HashMap::new(),
            body: String::new(),
        }
    }
    pub fn r_500() -> HttpResponse {
        HttpResponse {
            version: HttpVersion::Http1_1,
            status: HttpStatus::InternalServerError,
            headers: HashMap::new(),
            body: String::new(),
        }
    }
    pub fn r_404() -> HttpResponse {
        HttpResponse {
            version: HttpVersion::Http1_1,
            status: HttpStatus::NotFound,
            headers: HashMap::new(),
            body: String::new(),
        }
    }
    pub fn set_version(mut self, version: HttpVersion) -> Self {
        self.version = version;
        self
    }

    pub fn set_status(mut self, status: HttpStatus) -> Self {
        self.status = status;
        self
    }
    pub fn set_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
    pub fn build(self) -> HttpResponse {
        HttpResponse {
            version: self.version,
            status: self.status,
            headers: self.headers,
            body: self.body,
        }
    }

    pub fn set_body(mut self, body: String) -> Self {
        self = self.set_header("Content-Length".into(), format!("{}", body.len()));
        self.body = body;
        self
    }
}
