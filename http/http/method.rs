use std::fmt::Display;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Put,
    Delete,
    Options,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Post => write!(f, "POST"),
            Self::Patch => write!(f, "PATCH"),
            Self::Put => write!(f, "PUT"),
            Self::Delete => write!(f, "DELETE"),
            Self::Options => write!(f, "OPTIONS"),
        }
    }
}
