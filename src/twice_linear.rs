// https://www.codewars.com/kata/twice-linear

use std::collections::HashSet;

fn dbl_linear(n: u32) -> u32 {
  let mut x_index = 0;
  let mut y_index = 0;

  let mut u = vec![1];
  let mut cache = HashSet::new();
  cache.insert(1);

  while u.len() < (n+1) as usize {
    let x = u[x_index] * 2 + 1;
    let y = u[y_index] * 3 + 1;

    match x < y {
      true => {
        x_index += 1;
        if cache.insert(x) { u.push(x); }
      },
      _ => {
        y_index += 1;
        if cache.insert(y) { u.push(y); }
      }
    }

  }

  *u.last().unwrap()
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
  }
}
