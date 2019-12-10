// https://www.codewars.com/kata/54d512e62a5e54c96200019e

use std::collections::BTreeMap;

fn prime_factors(n: i64) -> String {
  // ref: https://www.geeksforgeeks.org/print-all-prime-factors-of-a-given-number/
  let mut cur = n.abs();
  let mut primes = BTreeMap::new();

  // handle all the 2's
  while cur % 2 == 0 {
    *primes.entry(2).or_insert(0) += 1;
    cur /= 2;
  }

  for i in (3..(cur as f64).sqrt() as i64 + 1).step_by(2) {
    while cur % i == 0 {
      *primes.entry(i).or_insert(0) += 1;
      cur /= i;
    }
  }

  if cur > 2 {
    *primes.entry(cur).or_insert(0) += 1;
  }

  println!("{:?}", primes);

  primes
    .iter()
    .map(|(n, c)| match c {
      &1 => format!("({})", n),
      _ => format!("({}**{})", n, c),
    })
    .collect::<String>()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn testing(n: i64, exp: &str) -> () {
    assert_eq!(&prime_factors(n), exp)
  }

  #[test]
  fn basics_prime_factors() {
    testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
    testing(17 * 17 * 93 * 677, "(3)(17**2)(31)(677)");
  }
}
