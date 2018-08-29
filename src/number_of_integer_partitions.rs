// https://www.codewars.com/kata/number-of-integer-partitions

use std::collections::HashSet;

fn partitions(n: isize) -> isize {
  if n == 1 { return 1 }

  let mut cur = vec![n];
  let mut seen = HashSet::new();

  while !cur.is_empty() {
    seen.insert(cur.clone());

    let tmp = cur.pop().unwrap() - 1;
    if tmp == 1 { continue; }
    cur.push(tmp);

    if !seen.contains(&cur) {
      let mut rem = n - cur.iter().sum::<isize>();
      if rem == 1 { continue }

      let last = *cur.last().unwrap();
      while rem > last {
        cur.push(last);
        rem -= last;
      }
      if rem == 1 { continue }
      cur.push(rem);
    }
  }

  seen.len() as isize + 1
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_partitions() {
    assert_eq!(partitions(1), 1);
    assert_eq!(partitions(2), 2);
    assert_eq!(partitions(3), 3);
    assert_eq!(partitions(4), 5);
    assert_eq!(partitions(5), 7);
    assert_eq!(partitions(10), 42);
    assert_eq!(partitions(100), 42);
  }
}
