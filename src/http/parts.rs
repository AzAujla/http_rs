#![allow(dead_code)]

// pub struct MutableList {
//     list: HashMap<String, String>,
// }

// impl MutableList {
//     fn new() -> Self {
//         return Self {
//             list: HashMap::new(),
//         };
//     }

//     fn has(&self, key: String) -> bool {
//         return self.list.contains_key(&key);
//     }

//     fn set(&mut self, key: String, value: String) -> String {
//         self.list.insert(key, value.clone());
//         return value;
//     }

//     fn unset(&mut self, key: String) -> bool {
//         if !self.list.contains_key(&key) {
//             return false;
//         } else {
//             self.list.remove(&key);

//             return true;
//         }
//     }
// }

#[derive(strum_macros::Display, Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
}

impl HttpMethod {
    pub fn from_str(str: &str) -> Self {
        match str {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "PATCH" => HttpMethod::PATCH,
            "DELETE" => HttpMethod::DELETE,
            "OPTIONS" => HttpMethod::OPTIONS,
            &_ => panic!("INVALID VERB"),
        }
    }
}

pub enum StatusCode {
    OK,
    CREATED,
    FORBIDDEN,
    NOTFOUND,
    UNAUTHORIZED,
    INTERNALSERVERERROR,
    OTHER,
}

impl StatusCode {
    pub fn to_string(&self) -> String {
        match self {
            Self::OK => "200 OK",
            Self::CREATED => "201 CREATED",
            StatusCode::FORBIDDEN => "403 FORBIDDEN",
            StatusCode::NOTFOUND => "404 NOTFOUND",
            StatusCode::UNAUTHORIZED => "401 UNAUTHORIZED",
            StatusCode::INTERNALSERVERERROR => "500 INTERNAL SERVER ERROR",
            Self::OTHER => "400 BAD REQUEST",
        }
        .to_string()
    }
}
