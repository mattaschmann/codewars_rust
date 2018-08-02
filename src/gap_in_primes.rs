fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {

  let mut gap = g + 1;
  for i in m..n+1 {
    if is_prime(i) {
      if gap == g { return Some((i-gap as u64, i)); }
      gap = 0;
    }
    gap += 1;
  }

  None
}

// Use the 'Optimized School Method'
fn is_prime(n: u64) -> bool {
  if n < 2 { return false; }
  if n <= 3 { return true; }

  if n % 2 == 0 || n % 3 == 0 { return false; }

  let mut i = 5;
  while i * i < n {
    if n % i == 0 || n % (i + 2) == 0 {
      return false;
    }
    i += 6;
  }

  true
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_gap() {
    assert_eq!(gap(1, 8, 9), None);
    assert_eq!(gap(2, 5, 7), Some((5, 7)));
    assert_eq!(gap(2,100,110), Some((101, 103)));
    assert_eq!(gap(4,100,110), Some((103, 107)));
    assert_eq!(gap(6,100,110), None);
    assert_eq!(gap(8,300,400), Some((359, 367)));
    assert_eq!(gap(11,30000,30011), None);
  }

  fn setup_is_prime(n: u64, result: bool) {
    println!("is {} prime?", n);
    assert_eq!(is_prime(n), result);
  }

  #[test]
  fn test_is_prime() {
    setup_is_prime(2, true);
    setup_is_prime(4, false);
    setup_is_prime(1, false);
    setup_is_prime(3, true);
    setup_is_prime(5, true);
    setup_is_prime(179425879, true);
    setup_is_prime(30000, false);
    setup_is_prime(30011, true);
  }
}
