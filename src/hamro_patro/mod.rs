use crate::client::default_client::DefaultClient;
use crate::http;
use crate::http::http::fetch;
use hyper::Body;

use self::date::NepaliDate;
use self::errors::ErrorKind;
use self::upcoming_events::Events;

pub mod date;
pub mod errors;
pub mod upcoming_events;

#[derive(Debug)]
pub struct HamroPatro {
  pub date: NepaliDate,
  pub events: Events,
}

impl HamroPatro {
  pub async fn fetch() -> Result<HamroPatro, ErrorKind> {
    let uri_builder = http::uri::Uri::builder()
      .scheme("https")
      .host("english.hamropatro.com");
    match uri_builder.build() {
      Ok(uri) => match fetch(
        DefaultClient::req_builder()
          .method("GET")
          .uri(uri)
          .body(Body::empty()),
      )
      .await
      {
        Ok(response) => {
          let text = response.to_string();

          match date::scrape_date(&text) {
            Ok(date) => {
              Ok(
                HamroPatro {
                  date,
                  events: upcoming_events::Events::scrape(&text)
                }
              )
            },
            Err(err) => Err(err)
          }
        }
        Err(_) => Err(ErrorKind::FetchError)
      },
      Err(_) => Err(ErrorKind::FetchError)
    }
  }
}
