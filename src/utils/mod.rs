pub fn sql_safe(input: String) -> String {
    let mut escaped = String::with_capacity(input.len() + 2);
    escaped.push('\'');
    for c in input.chars() {
        if c == '\'' {
            escaped.push_str("''"); // escape single quotes
        } else {
            escaped.push(c);
        }
    }
    escaped.push('\'');
    escaped
}
pub fn remove_trailing_slash(mut path: String) -> String {
    if path.len() > 1 && path.ends_with('/') {
        path.pop(); // remove the trailing '/'
    }
    path
}
