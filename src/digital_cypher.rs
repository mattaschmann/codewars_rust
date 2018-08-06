// https://www.codewars.com/kata/digital-cypher

fn encode(msg: String, n: i32) -> Vec<i32> {
  msg.chars()
     .zip(n.to_string().chars().cycle())
     .map(|(c, n)| c as i32 - 96 + n.to_digit(10).unwrap() as i32)
     .collect()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_encode() {
    assert_eq!(encode("a".to_string(), 0), [1]);
    assert_eq!(encode("a".to_string(), 1), [2]);
    assert_eq!(encode("scout".to_string(), 1939), vec![20, 12, 18, 30, 21]);
    assert_eq!(encode("masterpiece".to_string(), 1939), vec![14, 10, 22, 29, 6, 27, 19, 18, 6, 12, 8]);
  }
}
