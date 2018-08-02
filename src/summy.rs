fn summy(strng: &str) -> i32 {
  strng.split(' ')
       .map(|s| s.parse::<i32>().unwrap())
       .sum()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_summy() {
    assert_eq!(summy("1 2"), 3);
    assert_eq!(summy("3 4 5"), 12);
    assert_eq!(summy("1 2 3"), 6);
    assert_eq!(summy("1 2 3 4"), 10);
    assert_eq!(summy("1 2 3 4 5"), 15);
    assert_eq!(summy("10 10"), 20);
    assert_eq!(summy("0 0"), 0);
  }
}
