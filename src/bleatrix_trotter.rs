// https://www.codewars.com/kata/bleatrix-trotter-the-counting-sheep

use std::collections::HashSet;

fn trotter(n: i32)-> i32{
  if n == 0 { return -1; }

  let mut seen = HashSet::new();

  let mut cur = n;
  let mut coef = 1;
  while seen.len() < 10 {
    cur = n * coef;
    coef += 1;
    for x in cur.to_string().chars() { seen.insert(x); }
  }

  cur
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_trotter() {
    assert_eq!(trotter(0), -1);
    assert_eq!(trotter(1), 10);
    assert_eq!(trotter(2), 90);
    assert_eq!(trotter(7), 70);
    assert_eq!(trotter(1692), 5076);
  }
}
