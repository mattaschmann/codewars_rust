// https://www.codewars.com/kata/5a25ac6ac5e284cfbe000111

fn triangle(row_str: &str) -> String {
  if row_str.len() <= 1 {
    return row_str.to_string();
  };

  let res = row_str
    .chars()
    .collect::<Vec<char>>()
    .windows(2)
    .map(|x| match x {
      ['R', 'R'] | ['B', 'G'] | ['G', 'B'] => 'R',
      ['B', 'B'] | ['R', 'G'] | ['G', 'R'] => 'B',
      ['G', 'G'] | ['B', 'R'] | ['R', 'B'] => 'G',
      _ => panic!("Combination not found!"),
    })
    .collect::<String>();

  triangle(&res)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_triangle() {
    assert_eq!(triangle("B"), "B");
    assert_eq!(triangle("GB"), "R");
    assert_eq!(triangle("RRR"), "R");
    assert_eq!(triangle("RGBG"), "B");
    assert_eq!(triangle("RBRGBRB"), "G");
    assert_eq!(triangle("RBRGBRBGGRRRBGBBBGG"), "G");
    assert_eq!(triangle("GB"), "R");
  }
}
