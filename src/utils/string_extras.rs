pub trait StringExtra {
  fn remove_multiple_spaces(self) -> String;
  fn remove_newline_chars(self) -> String;
  fn space_between_alphanumeric(self) -> String;
  fn to_title_case(self) -> String;
  fn untrim_start(self, n: usize) -> String;
  fn untrim_end(self, n: usize) -> String;
  fn untrim(self, start: usize, end: usize) -> String;
}

impl StringExtra for String {
  fn remove_multiple_spaces(self) -> String {
    let mut proper_str = String::new();
    let mut prev_char = ' ';

    for char in self.chars() {
      if !(char.is_whitespace() && prev_char.is_whitespace()) {
        proper_str.push(char);
      }
      prev_char = char;
    }
    proper_str
  }

  fn remove_newline_chars(self) -> String {
    let mut proper_str = String::new();

    for char in self.chars() {
      if char == '\n' {
        continue;
      }
      proper_str.push(char);
    }
    proper_str
  }

  fn space_between_alphanumeric(self) -> String {
    let mut proper_str = String::new();
    let mut prev_char = ' ';

    for char in self.chars() {
      if prev_char.is_digit(10) && !char.is_digit(10) {
        proper_str.push(' ');
        proper_str.push(char);
      } else if !prev_char.is_digit(10) && char.is_digit(10) {
        proper_str.push(' ');
        proper_str.push(char);
      } else {
        proper_str.push(char);
      }
      prev_char = char;
    }
    proper_str
  }

  fn to_title_case(self) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for ch in self.chars() {
      if ch.is_whitespace() {
        capitalize_next = true;
        result.push(ch);
      } else {
        if capitalize_next {
          result.push(ch.to_uppercase().next().unwrap());
        } else {
          result.push(ch.to_lowercase().next().unwrap());
        }
        capitalize_next = false;
      }
    }
    result
  }

  fn untrim_start(self, n: usize) -> String {
    let mut s = String::new();
    for _ in 0..n {
      s.push(' ');
    }
    s.push_str(self.as_str());
    s
  }

  fn untrim_end(self, n: usize) -> String {
    let mut s = self.clone();
    for _ in 0..n {
      s.push(' ');
    }
    s
  }

  fn untrim(self, start: usize, end: usize) -> String {
    self.untrim_start(start).untrim_end(end)
  }


}
