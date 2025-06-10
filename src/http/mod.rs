mod method;
mod status_code;
mod version;
use std::collections::HashMap;

pub use method::HttpMethod;
pub use status_code::HttpStatus;
pub use version::HttpVersion;

pub trait DisplayHeaders {
    fn to_headers_string(self) -> String;
}

impl DisplayHeaders for HashMap<String, String> {
    fn to_headers_string(self) -> String {
        let mut headers = self
            .clone()
            .into_keys()
            .map(|e| format!("{}: {}", e, self.get(&e).unwrap()))
            .collect::<Vec<String>>();
        headers.reverse();
        headers.join("\r\n")
    }
}
