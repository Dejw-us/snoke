use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Method {
  Get,
  Post,
  Delete,
  Patch,
  Put,
  Head,
  Options,
  Connect,
}

impl FromStr for Method {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "get" => Ok(Method::Get),
      "post" => Ok(Method::Post),
      "delete" => Ok(Method::Delete),
      "patch" => Ok(Method::Patch),
      "put" => Ok(Method::Put),
      "head" => Ok(Method::Head),
      "options" => Ok(Method::Options),
      "connect" => Ok(Method::Connect),
      _ => Err(()),
    }
  }
}