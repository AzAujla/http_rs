pub enum HttpVersion {
    Http0_9,
    Http1_0,
    Http1_1,
    Http2_0,
    Http3_0,
}
impl HttpVersion {
    pub fn from(version: String) -> Result<Self, String> {
        match version.as_str() {
            "HTTP/0.9" => Ok(Self::Http0_9),
            "HTTP/1.0" => Ok(Self::Http1_0),
            "HTTP/1.1" => Ok(Self::Http1_1),
            "HTTP/2.0" => Ok(Self::Http2_0),
            "HTTP/3.0" => Ok(Self::Http3_0),
            _ => Err(format!("Unknown HTTP version; {}", version)),
        }
    }
}
impl std::fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpVersion::Http0_9 => write!(f, "HTTP/0.9"),
            HttpVersion::Http1_0 => write!(f, "HTTP/1.0"),
            HttpVersion::Http1_1 => write!(f, "HTTP/1.1"),
            HttpVersion::Http2_0 => write!(f, "HTTP/2.0"),
            HttpVersion::Http3_0 => write!(f, "HTTP/3.0"),
        }
    }
}
