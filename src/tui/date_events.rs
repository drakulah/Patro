use crate::{
  hamro_patro::{errors::ErrorKind, upcoming_events::EachEvent, HamroPatro},
  utils::{nep_month::NepaliMonth, string_extras::StringExtra, two_digit_str::two_digit_str},
};

pub enum DateFormat {
  MMDDYY,
  MMDDYYYY,
  YYMMDD,
  YYYYMMDD,
  DDMMYY,
  DDMMYYYY,
  NONE,
}

impl DateFormat {
  pub fn from_str(s: String) -> DateFormat {
    match s.as_ref() {
      "YYMMDD" => DateFormat::YYMMDD,
      "YYYYMMDD" => DateFormat::YYYYMMDD,
      "DDMMYY" => DateFormat::DDMMYY,
      "DDMMYYYY" => DateFormat::DDMMYYYY,
      "MMDDYY" => DateFormat::MMDDYY,
      "MMDDYYYY" => DateFormat::MMDDYYYY,
      _ => DateFormat::NONE,
    }
  }
}

pub struct DisplayOption {
  pub display_date: bool,
  pub display_events: bool,
  pub date_format: DateFormat,
  pub seperator: char,
}

fn change_len(s: String, l: usize) -> String {
  let n_s = s.clone();
  let n_l = n_s.len();
  if n_l == l {
    n_s
  } else if n_l > l {
    n_s[0..l].to_string()
  } else {
    n_s.untrim_end(l - n_l)
  }
}

fn last_two_digits(n: i16) -> String {
  let num_str = n.to_string();
  let len = num_str.len();
  num_str[len - 2..len].to_string()
}

pub async fn ui(opt: DisplayOption) {
  if !opt.display_date && !opt.display_events {
    return;
  }
  let front_spaces = "  ";
  println!("Fetching data from HamroPatro!");
  match HamroPatro::fetch().await {
    Ok(patro) => {
      if opt.display_date {
        let formatted = match opt.date_format {
          DateFormat::DDMMYY => format!(
            "{}{}{}{}{}",
            two_digit_str(patro.date.day),
            opt.seperator,
            two_digit_str(patro.date.month),
            opt.seperator,
            last_two_digits(patro.date.year)
          ),
          DateFormat::DDMMYYYY => format!(
            "{}{}{}{}{}",
            two_digit_str(patro.date.day),
            opt.seperator,
            two_digit_str(patro.date.month),
            opt.seperator,
            patro.date.year
          ),
          DateFormat::YYMMDD => format!(
            "{}{}{}{}{}",
            last_two_digits(patro.date.year),
            opt.seperator,
            two_digit_str(patro.date.month),
            opt.seperator,
            two_digit_str(patro.date.day),
          ),
          DateFormat::YYYYMMDD => format!(
            "{}{}{}{}{}",
            patro.date.year,
            opt.seperator,
            two_digit_str(patro.date.month),
            opt.seperator,
            two_digit_str(patro.date.day),
          ),
          DateFormat::MMDDYY => format!(
            "{}{}{}{}{}",
            two_digit_str(patro.date.month),
            opt.seperator,
            two_digit_str(patro.date.day),
            opt.seperator,
            last_two_digits(patro.date.year)
          ),
          DateFormat::MMDDYYYY => format!(
            "{}{}{}{}{}",
            two_digit_str(patro.date.month),
            opt.seperator,
            two_digit_str(patro.date.day),
            opt.seperator,
            patro.date.year
          ),
          _ => format!(
            "{}, {} {}, {}",
            patro.date.week_day,
            NepaliMonth::from_index(patro.date.month),
            patro.date.day,
            patro.date.year
          ),
        };
        println!("Date (Nepali) -> {}", formatted);

        if !patro.events.today.is_empty() {
          println!("\nToday's Events:");
          for e in patro.events.today.into_iter() {
            println!("{}{}", front_spaces, e);
          }
        }
      }

      if opt.display_events {
        let mut longest_date_len = 0;
        let mut events_vec: Vec<EachEvent> = Vec::new();

        for each_event in patro.events.upcoming_events.into_iter() {
          events_vec.push(each_event.clone());
          let l = each_event.date_text.len();
          if l > longest_date_len {
            longest_date_len = l
          }
        }

        let blank_len = longest_date_len + 4;
        if opt.display_date {
          println!("");
        }
        
        if events_vec.is_empty() {
          println!("Currently, there are no upcoming events nearby!");
        } else {
          println!("Upcoming Events:");
          for each_event in events_vec.into_iter() {
            let mut is_first = true;
            for title in each_event.events_title.into_iter() {
              if is_first {
                println!(
                  "{}{} -> {}",
                  front_spaces,
                  change_len(each_event.date_text.clone(), longest_date_len),
                  title
                );
                is_first = false;
              } else {
                println!(
                  "{}{}{}",
                  front_spaces,
                  String::new().untrim_end(blank_len),
                  title
                );
              }
            }
            println!(
              "{}{}({})",
              front_spaces,
              String::new().untrim_end(blank_len),
              each_event.starting_after
            );
          }
        }
      }
    }
    Err(err) => match err {
      ErrorKind::FetchError => println!("ERROR: Couldn't fetch the URL!"),
      ErrorKind::ParseError => println!("ERROR: Couldn't parse the HTML data!"),
    },
  }
}
