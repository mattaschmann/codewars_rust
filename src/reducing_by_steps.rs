fn som(x: i64, y: i64) -> i64 {
  x + y
}

fn maxi(x: i64, y: i64) -> i64 {
  if x > y {
    x
  } else {
    y
  }
}

fn mini(x: i64, y: i64) -> i64 {
  if x < y {
    x
  } else {
    y
  }
}

fn gcdi(m: i64, n: i64) -> i64 {
  let x = maxi(m.abs(), n.abs());
  let y = mini(m.abs(), n.abs());

  if x % y == 0 {
    y
  } else {
    gcdi(y, x % y)
  }
}

fn lcmu(a: i64, b: i64) -> i64 {
  (a * b).abs() / gcdi(a, b)
}

// first parameter: dots have to be replaced by function of two variables
fn oper_array(f: fn(i64, i64) -> i64, a: &[i64], init: i64) -> Vec<i64> {
  let mut out_array: Vec<i64> = Vec::new();

  a.iter().fold(init, |acc, x| {
    let current = f(acc, *x);
    out_array.push(current);
    current
  });

  out_array
}

#[test]
fn test_basics() {
  assert_eq!(som(1, 1), 2, "som should be the sum of the two inputs");
  assert_eq!(maxi(1, 2), 2, "maxi should be the highest of the two inputs");
  assert_eq!(mini(1, 2), 1, "mini should be the lowest of the two inputs");
  assert_eq!(gcdi(24, -54), 6, "gcdi should be the greatest common divisor of the two args");
  assert_eq!(lcmu(4, 6), 12, "lcmu should be the least common multiple of the two args");
}

#[test]
fn test_oper_array() {
  assert_eq!(oper_array(som, &[2, 4, 6, 8, 10, 20], 0), vec![2, 6, 12, 20, 30, 50]);
}
