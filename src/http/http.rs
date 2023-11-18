use hyper::{
  body::{to_bytes, Bytes},
  http::Error,
  Body, Request,
};

use super::errors::ErrorKind;

pub struct RequestInit {
  body_bytes: Bytes,
}

impl RequestInit {
  pub fn to_string(&self) -> String {
    let body_vec = self.body_bytes.to_vec();
    String::from_utf8_lossy(&body_vec).to_string()
  }
}

pub async fn fetch(req: Result<Request<Body>, Error>) -> Result<RequestInit, ErrorKind> {
  let req_client = super::client::default_client();

  match req {
    Ok(valid_req) => match req_client.request(valid_req).await {
      Ok(res) => match to_bytes(res.into_body()).await {
        Ok(body_bytes) => Ok(RequestInit { body_bytes }),
        Err(_) => Err(ErrorKind::BytesConversion),
      },
      Err(_) => Err(ErrorKind::ResponseBodyError),
    },
    Err(_) => Err(ErrorKind::InvalidRequest),
  }
}
