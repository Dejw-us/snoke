use std::collections::HashMap;
use std::str::FromStr;
use crate::method::Method;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
  method: Method,
  uri: String,
  version: String,
  headers: HashMap<String, String>,
  body: String,
}

impl Request {
  pub fn from_bytes(buf: &[u8]) -> Option<Self> {
    let text = std::str::from_utf8(buf).ok()?;

    let mut parts = text.splitn(2, "\r\n\r\n");
    let head = parts.next()?;
    let body = parts.next().unwrap_or("").to_string();

    let mut lines = head.lines();

    let request_line = lines.next()?;
    let mut request_line_parts = request_line.split_whitespace();
    let method_str = request_line_parts.next()?;
    let uri = request_line_parts.next()?.to_string();
    let version = request_line_parts.next()?.to_string();

    let method = Method::from_str(method_str).unwrap();

    let mut headers = HashMap::new();
    for line in lines {
      if let Some((key, value)) = line.split_once(":") {
        headers.insert(key.trim().to_string(), value.trim().to_string());
      }
    }

    Some(Self {
      method,
      uri,
      version,
      headers,
      body,
    })
  }
}
