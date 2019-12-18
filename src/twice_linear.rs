// https://www.codewars.com/kata/twice-linear

use std::collections::BTreeSet;

fn dbl_linear(n: u32) -> u32 {
  if n == 0 { return 1; }

  let mut u = BTreeSet::new();
  u.insert(1);

  for i in 0..(n-(n/3)) as usize {
    let x = *u.iter().nth(i).unwrap();

    u.insert(2*x+1);
    u.insert(3*x+1);
  }

  *u.iter().nth(n as usize).unwrap()
}

#[cfg(test)]
mod tests {
  use super::dbl_linear;
  fn testing(n: u32, exp: u32) -> () {
    time_test!(n);
    assert_eq!(dbl_linear(n), exp)
  }

  #[test]
  fn basics_dbl_linear() {
    testing(10, 22);
    testing(20, 57);
    testing(30, 91);
    testing(50, 175);
    testing(100, 447);
    testing(3000, 447);
  }
}
