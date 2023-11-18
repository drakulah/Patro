use clap::{arg, command, value_parser, ArgGroup};
use time_extra::nepali_time::NepaliTime;
use tui::date_events::DisplayOption;

use crate::{tui::date_events::DateFormat, utils::two_digit_str::two_digit_str};

mod client;
mod hamro_patro;
mod http;
mod time_extra;
mod tui;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let matches = command!()
    .group(ArgGroup::new("args").multiple(true))
    .next_help_heading("ARGS")
    .args([
      arg!(--time "Display current time").group("args"),
      arg!(--date "Display current date and event if available").group("args"),
      arg!(--events "Display upcoming events if available").group("args"),
      arg!(--sep <CHARACTER> "Valid date seperator character /, -").group("args").value_parser(value_parser!(char)),
      arg!(--datefmt <DATEFMT> "Format to display date in: MMDDYY, MMDDYYYY, YYMMDD, YYYYMMDD, DDMMYY, DDMMYYYY").group("args").value_parser(value_parser!(String)),
    ]).get_matches();

  let empty_str = String::new();

  let show_time = matches.get_flag("time");
  let show_date = matches.get_flag("date");
  let show_events = matches.get_flag("events");

  let seperator: char = *matches.get_one("sep").unwrap_or(&'/');
  let date_fmt = matches.get_one::<String>("datefmt").unwrap_or(&empty_str);

  if show_time {
    let time = NepaliTime::now();
    println!("Time (Nepali) -> {}:{}:{} {}", time.hours, two_digit_str(time.mins), two_digit_str(time.secs), time.meridiem);
  }

  if show_date || show_events {
    tui::date_events::ui(DisplayOption {
      display_date: show_date,
      display_events: show_events,
      seperator,
      date_format: DateFormat::from_str(date_fmt.clone())
    }).await;
  }

  Ok(())
}
