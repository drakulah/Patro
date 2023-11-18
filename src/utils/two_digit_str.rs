pub fn two_digit_str(n: i16) -> String {
  if n > 9 {
    n.to_string()
  } else {
    format!("0{}", n)
  }
}