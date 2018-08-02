use std::cmp::{min,max};

fn sum_fracts(l: Vec<(i64, i64)>) -> Option<(i64, i64)> {
  if l.is_empty() { return None; }

  let (large_n, large_d) = l.iter().fold((0, 1), |(x, y), &(n, d)| {
    let new_n = (x * d) + (n * y);
    let new_d = y * d;

    (new_n, new_d)
  });

  let gcd = gcdi(large_n, large_d);

  Some((large_n / gcd, large_d / gcd))
}
fn gcdi(m: i64, n: i64) -> i64 {
  let x = max(m.abs(), n.abs());
  let y = min(m.abs(), n.abs());

  if x % y == 0 {
    y
  } else {
    gcdi(y, x % y)
  }
}



#[test]
fn test_sum_fracts() {
  assert_eq!(sum_fracts(vec![]), None);
  assert_eq!(sum_fracts(vec![(1, 4), (1, 4)]), Some((1, 2)));
  assert_eq!(sum_fracts(vec![(1, 2), (1, 3), (1, 4)]), Some((13, 12)));
}
