// https://www.codewars.com/kata/54d496788776e49e6b00052f

use std::collections::{HashMap, HashSet};

fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
  let mut map = HashMap::new();

  // ref: https://www.geeksforgeeks.org/print-all-prime-factors-of-a-given-number/
  for n in l {
    let mut cur = n;
    // handle all the 2's
    while cur % 2 == 0 {
      map.entry(2).or_insert(HashSet::new()).insert(n);
      cur /= 2;
    }

    for i in (3..(cur as f64).sqrt() as i64 + 1).step_by(2) {
      while cur % i == 0 {
        map.entry(i).or_insert(HashSet::new()).insert(n);
        cur /= i;
      }
    }

    if cur > 2 {
      map.entry(cur).or_insert(HashSet::new()).insert(n);
    }
  }

  let mut res: Vec<(i64, i64)> = map.iter().map(|(k, set)| (*k, set.iter().sum())).collect();

  res.sort();

  res
}

#[cfg(test)]
mod tests {
  use super::*;

  fn testing(l: Vec<i64>, exp: Vec<(i64, i64)>) -> () {
    assert_eq!(sum_of_divided(l), exp)
  }

  #[test]
  fn basics_sum_of_divided() {
    testing(vec![12, 15], vec![(2, 12), (3, 27), (5, 15)]);
    testing(
      vec![15, 21, 24, 30, 45],
      vec![(2, 54), (3, 135), (5, 90), (7, 21)],
    );
  }
}
