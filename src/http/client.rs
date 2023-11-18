use hyper::{client::HttpConnector, Client};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};

pub fn default_client() -> Client<HttpsConnector<HttpConnector>> {
  let https_connector = HttpsConnectorBuilder::default()
    .with_native_roots()
    .https_or_http()
    .enable_http1()
    .build();

  Client::builder().build::<_, hyper::Body>(https_connector)
}
