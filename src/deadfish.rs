fn parse(code: &str) -> Vec<i32> {
  let mut ret = Vec::new();

  code.chars()
      .fold(0, |acc: i32, c| {
        match c {
          'i' => acc + 1,
          'd' => acc - 1,
          's' => acc.pow(2),
          'o' => {
            ret.push(acc);
            acc
          },
          _ => acc
        }
      });

  ret
}


#[test]
fn test_parse() {
  assert_eq!(parse(""), []);
  assert_eq!(parse("io"), [1]);
  assert_eq!(parse("do"), [-1]);
  assert_eq!(parse("so"), [0]);
  assert_eq!(parse("iiso"), [4]);
}
