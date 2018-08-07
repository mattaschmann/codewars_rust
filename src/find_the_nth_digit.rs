fn find_digit(num: i32, nth: i32) -> i32 {
  if nth < 1 { return -1 }

  match num.abs()
           .to_string()
           .chars()
           .rev()
           .skip(nth as usize - 1)
           .next() {
             Some(n) => n.to_digit(10).unwrap() as i32,
             _ => 0
           }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_find_digit() {
    assert_eq!(find_digit(1, 0), -1);
    assert_eq!(find_digit(1, 1), 1);
    assert_eq!(find_digit(5432, 3), 4);
    assert_eq!(find_digit(32, 5), 0);
    assert_eq!(find_digit(24, -8), -1);
    assert_eq!(find_digit(-24, 2), 2);
  }
}
