use chrono::{DateTime, Timelike, Utc};

#[derive(Debug)]
pub struct NepaliTime {
  pub hours: i16,
  pub mins: i16,
  pub secs: i16,
  pub meridiem: String,
}

impl NepaliTime {
  pub fn now() -> NepaliTime {
    let u_ts = Utc::now().timestamp();
    let diff = 20700;
    let n_ts = DateTime::from_timestamp(u_ts + diff, 0).unwrap();
    let mut hrs = n_ts.hour() as i16;
    let meridiem;

    if hrs > 12 {
      hrs = hrs - 12;
      meridiem = "PM";
    } else {
      meridiem = "AM";
    }

    NepaliTime {
      hours: hrs,
      mins: n_ts.minute() as i16,
      secs: n_ts.second() as i16,
      meridiem: meridiem.to_string()
    }
  }
}
