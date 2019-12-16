// https://www.codewars.com/kata/backwards-read-primes

use std::collections::HashSet;

fn backwards_prime(start: u64, stop: u64) -> Vec<u64> {
  println!("{} {}", start, stop);

  let mut res = Vec::new();
  let mut memoize = HashSet::new();

  for x in start.max(11)..stop+1 {
    if memoize.contains(&x) {
      res.push(x);
      continue;
    }

    let rev = reverse_number(x);
    if is_prime(x) && is_prime(rev) && x != rev {
      memoize.insert(rev);
      res.push(x);
    }
  }

  res
}

// Use the 'Optimized School Method'
fn is_prime(n: u64) -> bool {
  if n < 2 {
    return false;
  }
  if n <= 3 {
    return true;
  }

  if n % 2 == 0 || n % 3 == 0 {
    return false;
  }

  let mut i = 5;
  while i * i <= n {
    if n % i == 0 || n % (i + 2) == 0 {
      return false;
    }
    i += 6;
  }

  true
}

fn reverse_number(n: u64) -> u64 {
  n.to_string()
    .chars()
    .rev()
    .collect::<String>()
    .parse::<u64>()
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_prime() {
    assert_eq!(is_prime(7921), false);
  }

  #[test]
  fn test_reverse_number() {
    assert_eq!(reverse_number(5), 5);
    assert_eq!(reverse_number(13), 31);
    assert_eq!(reverse_number(11), 11);
  }

  fn testing(start: u64, stop: u64, exp: Vec<u64>) -> () {
    assert_eq!(backwards_prime(start, stop), exp)
  }

  #[test]
  fn tests_backwards_prime() {
    let a = vec![13, 17, 31, 37, 71, 73, 79, 97];
    testing(1, 100, a);
    let a = vec![13, 17, 31];
    testing(1, 31, a);
    let a = vec![1103, 1109, 1151, 1153, 1181, 1193, 1201, 1213, 1217, 1223, 1229, 1231, 1237, 1249, 1259, 1279, 1283];
    testing(1099, 1299, a);
  }
}
