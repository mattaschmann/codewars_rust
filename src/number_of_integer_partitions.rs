// https://www.codewars.com/kata/number-of-integer-partitions

fn partitions(n: isize) -> isize {
  let mut cur = vec![n];
  let mut count = 1;

  while !cur.is_empty() {
    println!("{:?}", cur);
    if cur.last().unwrap() == &1 {
      cur.pop().unwrap();
      continue;
    }

    let tmp = cur.pop().unwrap() - 1;
    cur.push(tmp);

    if cur.len() == 1 {
      let mut rem = n - tmp;
      let first = cur[0];
      while rem > first {
        cur.push(first);
        rem -= first;
      }
      cur.push(rem);
    }

    println!("{}", "count");
    count += 1;
  }

  println!("{}", "done");
  count
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_partitions() {
    // assert_eq!(partitions(1), 1);
    // assert_eq!(partitions(2), 2);
    // assert_eq!(partitions(3), 3);
    // assert_eq!(partitions(4), 5);
    // assert_eq!(partitions(5), 7);
    // assert_eq!(partitions(7), 14);
    assert_eq!(partitions(10), 42);
  }
}
