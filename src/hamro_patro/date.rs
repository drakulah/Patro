use crate::utils::nep_month::NepaliMonth;

use super::errors::ErrorKind;

#[derive(Debug)]
pub struct NepaliDate {
  pub day: i16,
  pub month: i16,
  pub year: i16,
  pub week_day: String,
}

pub fn scrape_date(html_content: &str) -> Result<NepaliDate, ErrorKind> {
  let selector = scraper::Selector::parse("#top .date > .nep").unwrap();
  let document = scraper::Html::parse_document(html_content);
  let date_node = document
    .select(&selector)
    .next()
    .unwrap()
    .text()
    .collect::<String>();
  let trimmed = date_node.trim().replace(",", "");
  let splitted: Vec<&str> = trimmed.split(' ').collect();

  if splitted.len() != 4 {
    return Err(ErrorKind::ParseError);
  }

  let day = splitted[0].parse::<i16>().unwrap_or(0);
  let month = NepaliMonth::from_title(splitted[1]);
  let year = splitted[2].parse::<i16>().unwrap_or(0);
  let week_day = splitted[3].to_string();

  Ok(NepaliDate {
    day,
    month,
    year,
    week_day,
  })
}
