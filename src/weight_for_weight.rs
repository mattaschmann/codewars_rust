// https://www.codewars.com/kata/weight-for-weight

use std::cmp::Ordering;

fn order_weight(s: &str) -> String {
  let weight = |x: &str| x.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();

  let mut nums: Vec<&str> = s
    .split_whitespace()
    .collect();

  nums.sort_unstable_by(|a, b| {
    let res = weight(a).cmp(&weight(b));
    match res {
      Ordering::Equal => a.cmp(b),
      _ => res
    }
  });
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
