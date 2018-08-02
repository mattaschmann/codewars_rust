use std::cmp::*;

/** Explaination from comments:
 *
 * For any number m, a^b mod m will
 * always have a period of a factor of phi(m) (phi is the Euler totient
 * function). phi(10) = 4. This means that any results bigger than 10 will
 * be going around a loop inside the modular arithmetic with a period of 4,
 * so you can cut off all the extra loops.
 *
 * There's an edge case though: if a^b hasn't reached m then the above
 * derivation is invalid. But this should just need a very simple check ;-)
 */
fn last_digit(lst: &[u64]) -> u64 {
  let f_mod = |x, m| min(x % m + m, x);

  lst.iter()
     .rev()
     .fold(1, |acc, &n| f_mod(n, 20).pow(f_mod(acc, 4) as u32)) % 10
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_last_digit() {
    let tests = vec![
      (vec![], 1),
      (vec![0, 0], 1),
      (vec![0, 0, 0], 0),
      (vec![7, 1], 7),
      (vec![8, 8], 6),
      (vec![9, 9], 9),
      (vec![1, 2], 1),
      (vec![3, 4, 5], 1),
      (vec![4, 3, 6], 4),
      (vec![6, 21], 6),
      (vec![7, 6], 9),
      (vec![7, 6, 21], 1),
      (vec![12, 30, 21], 6),
      (vec![4, 4, 4, 0], 6),
      (vec![3, 3, 3, 0], 7),
      (vec![1, 1, 1, 0], 1),
      (vec![2, 2], 4),
      (vec![2, 2, 2], 6),
      (vec![2, 2, 2, 1], 6),
      (vec![2, 1], 2),
      (vec![2, 2, 1], 4),
      (vec![2, 2, 2, 0], 4),
      (vec![937640, 767456, 981242], 0),
      (vec![123232, 694022, 140249], 6),
      (vec![499942, 898102, 846073], 6),
      (vec![2, 2, 101, 2], 6),
      (vec![2, 2, 1001, 2], 6),
      (vec![12, 18], 4),
    ];

    for test in tests {
      assert_eq!(last_digit(&test.0), test.1);
    }
  }
}
