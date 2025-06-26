use std::{collections::HashMap, fmt::Display};

use serde::Serialize;
use serde_json::value::Serializer;

use crate::http::status::HttpStatusCode;

pub struct Response {
    status: HttpStatusCode,
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
    body: String,
}

impl Response {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ReponseBuilder {
        ReponseBuilder {
            status: HttpStatusCode::OK,
            headers: HashMap::from([("Content-Length".into(), "0".into())]),
            cookies: HashMap::new(),
            body: String::new(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub path: Option<String>,
    pub domain: Option<String>,
    pub max_age: Option<u64>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<String>,
}

impl From<&str> for Cookie {
    fn from(value: &str) -> Self {
        let parts = value.split(';').map(str::trim);
        let mut cookie = Cookie {
            name: String::new(),
            value: String::new(),
            path: None,
            domain: None,
            max_age: None,
            secure: false,
            http_only: false,
            same_site: None,
        };

        for (i, part) in parts.enumerate() {
            if i == 0 {
                // First part is name=value
                let mut kv = part.splitn(2, '=');
                cookie.name = kv.next().unwrap().to_string();
                cookie.value = kv.next().unwrap().to_string();
            } else {
                // Remaining parts are attributes
                let mut kv = part.splitn(2, '=');
                let key = kv.next().unwrap().to_ascii_lowercase();
                let value = kv.next();

                match key.as_str() {
                    "path" => cookie.path = value.map(str::to_string),
                    "domain" => cookie.domain = value.map(str::to_string),
                    "max-age" => {
                        if let Some(v) = value {
                            if let Ok(secs) = v.parse() {
                                cookie.max_age = Some(secs);
                            }
                        }
                    }
                    "samesite" => cookie.same_site = value.map(str::to_string),
                    "secure" => cookie.secure = true,
                    "httponly" => cookie.http_only = true,
                    _ => {} // ignore unknown attributes
                }
            }
        }

        cookie
    }
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct CookieBuilder {
    name: String,
    value: String,
    path: Option<String>,
    domain: Option<String>,
    max_age: Option<u64>,
    secure: bool,
    http_only: bool,
    same_site: Option<String>,
}

impl CookieBuilder {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            ..Default::default()
        }
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    pub fn max_age(mut self, seconds: u64) -> Self {
        self.max_age = Some(seconds);
        self
    }

    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    pub fn http_only(mut self, http_only: bool) -> Self {
        self.http_only = http_only;
        self
    }

    pub fn same_site(mut self, policy: impl Into<String>) -> Self {
        self.same_site = Some(policy.into());
        self
    }

    pub fn build(self) -> Cookie {
        Cookie {
            name: self.name,
            value: self.value,
            path: self.path,
            domain: self.domain,
            max_age: self.max_age,
            secure: self.secure,
            http_only: self.http_only,
            same_site: self.same_site,
        }
    }
}

impl Display for Cookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.cookies.is_empty() {
            write!(
                f,
                "{}\r\n{}\r\n\r\n{}",
                self.status,
                self.headers
                    .iter()
                    .map(|(a, b)| { format!("{}: {}", a, b) })
                    .collect::<Vec<String>>()
                    .join("\r\n"),
                self.body
            )
        } else {
            write!(
                f,
                "{}\r\n{}Set-Cookie: {}\r\n\r\n{}",
                self.status,
                self.headers
                    .iter()
                    .map(|(a, b)| { format!("{}: {}", a, b) })
                    .collect::<Vec<String>>()
                    .join("\r\n"),
                self.cookies
                    .iter()
                    .map(|(a, b)| format!("{}={}", a, b))
                    .collect::<Vec<String>>()
                    .join("; "),
                self.body
            )
        }
    }
}

#[derive(Clone)]
pub struct ReponseBuilder {
    status: HttpStatusCode,
    headers: HashMap<String, String>,
    cookies: HashMap<String, Cookie>,
    body: String,
}

impl ReponseBuilder {
    pub fn status(mut self, status: HttpStatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn set_header(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn header(mut self, k: &'static str, v: &'static str) -> Self {
        self.headers.insert(String::from(k), String::from(v));
        self
    }

    pub fn get_header(&self, k: &'static str) -> Option<&String> {
        self.headers.get(k)
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.headers
    }

    pub fn cookie(mut self, k: &'static str, v: Cookie) -> Self {
        self.cookies.insert(String::from(k), v);
        self
    }

    pub fn get_cookie(&self, k: &'static str) -> Option<&Cookie> {
        self.cookies.get(k)
    }

    pub fn cookies(&self) -> &HashMap<String, Cookie> {
        &self.cookies
    }

    pub fn cookies_mut(&mut self) -> &mut HashMap<String, Cookie> {
        &mut self.cookies
    }

    pub fn body<T: ToString>(mut self, body: T) -> Self {
        self.body = body.to_string();
        self.headers
            .insert(String::from("Content-Type"), "text/plain".to_string());
        self.headers.insert(
            String::from("Content-Length"),
            format!("{}", self.body.len()),
        );
        self
    }

    pub fn content_type(mut self, content: &str) -> Self {
        self.headers
            .insert("Content-Type".into(), String::from(content));
        self
    }

    pub fn json<T: Serialize>(mut self, body: T) -> Self {
        self.body = body.serialize(Serializer).unwrap().to_string();
        self.headers
            .insert("Content-Length".into(), format!("{}", self.body.len()));
        self.headers
            .insert("Content-Type".into(), String::from("application/json"));
        self
    }

    pub fn build(self) -> Response {
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| (k.to_owned(), v.clone()))
            .collect();

        let cookies = self
            .cookies
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_string()))
            .collect();

        Response {
            status: self.status,
            body: self.body,
            headers,
            cookies,
        }
    }
}
