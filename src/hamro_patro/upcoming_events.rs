use crate::utils::string_extras::StringExtra;

#[derive(Debug, Clone)]
pub struct EachEvent {
  pub date_text: String,
  pub starting_after: String, // In Days
  pub events_title: Vec<String>,
}

#[derive(Debug)]
pub struct Events {
  pub today: Vec<String>,
  pub upcoming_events: Vec<EachEvent>,
}

impl Events {
  pub fn scrape(html_content: &str) -> Events {
    let document = scraper::Html::parse_document(html_content);

    let container_node_selector =
      scraper::Selector::parse("#content .upcomingdays.scroll > li").unwrap();
    let date_node_selector = scraper::Selector::parse(".date").unwrap();
    let title_node_selector = scraper::Selector::parse(".info > span > a").unwrap();
    let remaining_node_selector = scraper::Selector::parse(".info").unwrap();

    let mut today_events: Vec<String> = Vec::new();
    let mut upcoming_events: Vec<EachEvent> = Vec::new();

    for container_node in document.select(&container_node_selector).into_iter() {
      let date_node = container_node.select(&date_node_selector).next();
      let title_node = container_node.select(&title_node_selector).next();
      let remaining_node = container_node.select(&remaining_node_selector).next();

      if date_node.is_none() || title_node.is_none() || remaining_node.is_none() {
        continue;
      }

      let date_value = date_node
        .unwrap()
        .text()
        .collect::<String>()
        .trim()
        .to_string()
        .space_between_alphanumeric()
        .trim()
        .to_string();
      let mut title_value = title_node
        .unwrap()
        .text()
        .collect::<String>()
        .trim()
        .to_string();
      let remaining_value = remaining_node
        .unwrap()
        .text()
        .collect::<String>()
        .replace(&title_value, "")
        .trim()
        .to_string()
        .remove_multiple_spaces()
        .remove_newline_chars()
        .trim()
        .to_string()
        .to_title_case();
      title_value = title_value
        .remove_multiple_spaces()
        .remove_newline_chars()
        .trim()
        .to_string();
      let mut event_titles: Vec<String> = Vec::new();

      for title in title_value.split("/") {
        event_titles.push(title.trim().to_string());
      }

      if remaining_value.to_lowercase() == "today" {
        today_events = event_titles;
      } else {
        upcoming_events.push(EachEvent {
          date_text: date_value,
          starting_after: remaining_value,
          events_title: event_titles,
        });
      }
    }

    Events {
      today: today_events,
      upcoming_events,
    }
  }
}
