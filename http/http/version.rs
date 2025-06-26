#![allow(non_camel_case_types)]
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HttpVersion {
    HTTP_0_9,
    HTTP_1_0,
    HTTP_1_1,
    HTTP_2_0,
    HTTP_3_0,
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpVersion::HTTP_0_9 => write!(f, "HTTP/0.9"),
            HttpVersion::HTTP_1_0 => write!(f, "HTTP/1.0"),
            HttpVersion::HTTP_1_1 => write!(f, "HTTP/1.1"),
            HttpVersion::HTTP_2_0 => write!(f, "HTTP/2.0"),
            HttpVersion::HTTP_3_0 => write!(f, "HTTP/3.0"),
        }
    }
}
