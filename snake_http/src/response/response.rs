use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
  pub version: String,
  pub status_code: u16,
  pub reason_phrase: String,
  pub headers: HashMap<String, String>,
  pub body: String,
}

impl Response {
  /// Creates a new Response with default HTTP/1.1 version and empty headers/body
  pub fn new(status_code: u16, reason_phrase: impl Into<String>, body: impl Into<String>) -> Self {
    Self {
      version: "HTTP/1.1".to_string(),
      status_code,
      reason_phrase: reason_phrase.into(),
      headers: HashMap::new(),
      body: body.into(),
    }
  }

  /// Set a header
  pub fn set_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
    self.headers.insert(key.into(), value.into());
    self
  }

  /// Convert the Response to a raw HTTP string
  pub fn to_string(&self) -> String {
    let mut response = format!(
      "{} {} {}\r\n",
      self.version, self.status_code, self.reason_phrase
    );

    for (k, v) in &self.headers {
      response.push_str(&format!("{}: {}\r\n", k, v));
    }

    response.push_str("\r\n");
    response.push_str(&self.body);
    response
  }
}
