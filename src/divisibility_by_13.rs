fn thirt(n: i64) -> i64 {
  let pattern = vec![1, 10, 9, 12, 3, 4];
  let mut pattern_it = pattern.iter().cycle();

  let result: i64 = n.to_string().chars().rev().fold(0, |sum, c| {
    let cur_pat = pattern_it.next().unwrap();
    let cur_d = c.to_digit(10).unwrap() as i64;

    sum + (cur_pat * cur_d)
  });

  if result != n {
    thirt(result)
  } else {
    result
  }
}


#[test]
fn test_thirt() {
  assert_eq!(thirt(8529), 79);
  assert_eq!(thirt(85299258), 31);
  assert_eq!(thirt(5634), 57);
}
