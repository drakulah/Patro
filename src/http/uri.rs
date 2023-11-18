use hyper::http::Error;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Uri {
  scheme: String,
  host: String,
  path: String,
  search_params: HashMap<String, String>,
}

impl Uri {
  pub fn builder() -> Uri {
    Uri {
      scheme: "https".to_string(),
      host: "".to_string(),
      path: "/".to_string(),
      search_params: HashMap::new(),
    }
  }

  pub fn scheme(&mut self, scheme: &str) -> Uri {
    self.scheme = scheme.trim().to_string();
    self.clone()
  }

  pub fn host(&mut self, host: &str) -> Uri {
    self.host = host.trim().to_string();
    self.clone()
  }

  pub fn build(&self) -> Result<hyper::Uri, Error> {
    let mut path_and_query = String::from(self.path.as_str());

    if !self.search_params.is_empty() {
      path_and_query.push('?');
    }

    for (k, v) in self.search_params.iter() {
      path_and_query.push_str(format!("{}={}&", k, v).as_str());
    }

    if !self.search_params.is_empty() {
      path_and_query.pop();
    }

    hyper::Uri::builder()
      .scheme(self.scheme.as_str())
      .authority(self.host.as_str())
      .path_and_query(path_and_query)
      .build()
  }
}
