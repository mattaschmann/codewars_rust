// https://www.codewars.com/kata/weight-for-weight

fn order_weight(s: &str) -> String {
  // @Matt TODO: current
  let mut nums: Vec<String> = s
    .split_whitespace()
    .map(|n| {
      let sum: u32 = n
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum();

      sum.to_string()
    })
    .collect();

  nums.sort_unstable();
  nums.join(" ")
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_order_weight() {
    let tests = vec![
      ("100", "100"),
      ("100 101", "100 101"),
      ("101 100", "100 101"),
      ("103 123 4444 99 2000", "2000 103 123 4444 99"),
      ("2000 10003 1234000 44444444 9999 11 11 22 123", "11 11 2000 10003 22 123 1234000 44444444 9999"),
    ];

    for test in tests {
      assert_eq!(order_weight(&test.0), test.1);
    }
  }
}
