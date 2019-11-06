// https://www.codewars.com/kata/59a96d71dbe3b06c0200009c

fn generate_shape(n: i32) -> String {
  let line = "+".repeat(n as usize);

  std::iter::repeat(line)
    .take(n as usize)
    .collect::<Vec<String>>()
    .join("\n")
}

#[test]
fn sample_test() {
  assert_eq!(generate_shape(3), "+++\n+++\n+++");
}
