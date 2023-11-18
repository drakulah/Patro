pub struct NepaliMonth {}

impl NepaliMonth {
  pub fn from_index(index: i16) -> String {
    match index {
      1 => "Baishakh",
      2 => "Jestha",
      3 => "Aashadha",
      4 => "Shrawan",
      5 => "Bhadra",
      6 => "Ashwin",
      7 => "Kartik",
      8 => "Mangsir",
      9 => "Paush",
      10 => "Magh",
      11 => "Falgun",
      12 => "Chaitra",
      _ => "None",
    }
    .to_string()
  }

  pub fn from_title<S: AsRef<str>>(title: S) -> i16 {
    match title.as_ref() {
      "Baishakh" => 1,
      "Jestha" => 2,
      "Aashadha" => 3,
      "Shrawan" => 4,
      "Bhadra" => 5,
      "Ashwin" => 6,
      "Kartik" => 7,
      "Mangsir" => 8,
      "Paush" => 9,
      "Magh" => 10,
      "Falgun" => 11,
      "Chaitra" => 12,
      _ => 0,
    }
  }
}
