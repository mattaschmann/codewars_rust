// http://www.codewars.com/kata/526571aae218b8ee490006f4

fn count_bits(n: i64) -> u32 {
  format!("{:b}", n)
    .chars()
    .filter(|c| c == &'1')
    .count() as u32
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_count_bits() {
    assert_eq!(count_bits(0), 0);
    assert_eq!(count_bits(1), 1);
    assert_eq!(count_bits(2), 1);
    assert_eq!(count_bits(3), 2);
  }
}
