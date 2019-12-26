// https://www.codewars.com/kata/color-choice

extern crate num;
use self::num::{BigUint, One};

fn check_choose(m: u64, n: u64) -> i64 {
  println!("m: {}, n: {}", m, n);
  let mut colors = 0;

  while binomial_coef(n, colors) != m.into() {
    colors += 1;
    if colors > n { return -1; }
  }

  colors as i64
}

fn binomial_coef(n: u64, x: u64) -> BigUint {
  factorial(n.into()) / (factorial(x.into()) * factorial((n - x).into()))
}

fn factorial(n: u64) -> BigUint {
  (1..n+1).fold(BigUint::one(), |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_factorial() {
    assert_eq!(factorial(4), BigUint::from(24u64));
    assert_eq!(factorial(7), BigUint::from(5040u64));
    assert_eq!(factorial(25), BigUint::from(15511210043330985984000000u128));
    assert_eq!(factorial(48), BigUint::parse_bytes(b"12413915592536072670862289047373375038521486354677760000000000", 10).unwrap());
  }

  #[test]
  fn test_binomial_coef() {
    assert_eq!(binomial_coef(4, 2), BigUint::from(6u32));
    assert_eq!(binomial_coef(7, 2), BigUint::from(21u32));
  }

  fn dotest(m: u64, n: u64, exp: i64) -> () {
    assert_eq!(check_choose(m, n), exp)
  }
  #[test]
  fn basics_check_choose() {
    dotest(6, 4, 2);
    dotest(4, 4, 1);
    dotest(35, 7, 3);
    dotest(36, 7, -1);
    dotest(184756, 20, 10);
    dotest(3268760, 25, 10);
  }

}
