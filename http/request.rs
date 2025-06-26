use std::{collections::HashMap, str::FromStr};

use crate::{
    http::{method::HttpMethod, version::HttpVersion},
    response::Cookie,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub method: HttpMethod,
    pub version: HttpVersion,
    pub uri: String,
    pub get_string: String,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, Cookie>,
    pub body: String,
}

impl FromStr for Request {
    type Err = serde::de::value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().peekable();
        let method: HttpMethod;

        match chars.next().unwrap() {
            'G' => {
                method = HttpMethod::Get;
                chars.next(); // E
                chars.next(); // T
                chars.next(); // 
            }
            'P' => {
                match chars.next().unwrap() {
                    'O' => {
                        method = HttpMethod::Post;
                        chars.next(); //S
                        chars.next(); //T
                        chars.next(); //
                    }
                    'U' => {
                        method = HttpMethod::Put;
                        chars.next(); //T
                        chars.next(); //
                    }
                    _ => {
                        method = HttpMethod::Patch;
                        chars.next(); //T
                        chars.next(); //C
                        chars.next(); //H
                        chars.next(); //
                    }
                }
            }
            'D' => {
                method = HttpMethod::Delete;
                chars.next(); //E
                chars.next(); //L
                chars.next(); //E
                chars.next(); //T
                chars.next(); //E
                chars.next(); // 
            }
            _ => {
                method = HttpMethod::Options; //O
                chars.next(); //P
                chars.next(); //T
                chars.next(); //I
                chars.next(); //O
                chars.next(); //N
                chars.next(); //S
                chars.next(); // 
            }
        }

        let mut uri = String::new();
        let mut get_string = String::new();
        let mut uri_done = false;

        while let Some(&c) = chars.peek() {
            if c == '?' {
                break;
            } else if c == ' ' {
                uri_done = true;
                break;
            }
            uri.push(chars.next().unwrap());
        }

        if !uri_done {
            chars.next();
            while let Some(&c) = chars.peek() {
                if c == ' ' {
                    break;
                }
                get_string.push(chars.next().unwrap());
            }
        }

        chars.next(); //  
        chars.next(); // H
        chars.next(); // T
        chars.next(); // T
        chars.next(); // P
        chars.next(); // /

        let version: HttpVersion;
        match chars.next().unwrap() {
            '0' => {
                version = HttpVersion::HTTP_0_9;
                chars.next(); //.
                chars.next(); //9
            }
            '1' => {
                chars.next();
                match chars.next().unwrap() {
                    '0' => {
                        version = HttpVersion::HTTP_1_0;
                    }
                    _ => {
                        version = HttpVersion::HTTP_1_1;
                    }
                }
            }
            '2' => {
                version = HttpVersion::HTTP_2_0;
                chars.next(); //.
                chars.next(); //0
            }
            _ => {
                version = HttpVersion::HTTP_3_0;
                chars.next(); //.
                chars.next(); //0
            }
        }
        chars.next(); //\r
        chars.next(); //\n

        let mut headers = HashMap::new();
        let mut cookies = HashMap::new();

        let mut buffer1 = String::new();
        let mut buffer2 = String::new();

        let mut state = false; // or use enum

        while let Some(c) = chars.next() {
            match c {
                ':' if !state => {
                    state = true;
                    while chars.peek() == Some(&' ') {
                        chars.next();
                    }
                }
                '\r' => {
                    if chars.peek() == Some(&'\n') {
                        chars.next();

                        if !buffer1.is_empty() {
                            match buffer1.as_str() {
                                "Cookie" => {
                                    cookies.insert(buffer2.clone(), Cookie::from(buffer1.trim()));
                                }
                                _ => {
                                    headers.insert(buffer2.clone(), buffer1.trim().to_string());
                                }
                            }
                            buffer1.clear();
                            buffer2.clear();
                            state = false;
                        } else {
                            break;
                        }
                    }
                }
                c => {
                    if state {
                        buffer1.push(c);
                    } else {
                        buffer2.push(c);
                    }
                }
            }
        }

        let body: String = chars.collect();

        Ok(Self {
            method,
            uri,
            get_string,
            version,
            headers,
            cookies,
            body,
        })
    }
}
