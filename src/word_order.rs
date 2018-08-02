fn order(sentence: &str) -> String {
  let mut str_vec: Vec<&str> = sentence.split_whitespace().collect();

  str_vec.sort_by_key(|s| s.trim_matches(char::is_alphabetic));

  str_vec.join(" ")
}

#[test]
fn returns_expected() {
  assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
  assert_eq!(order("is200 Thi12s T488908est -23a"), "-23a Thi12s is200 T488908est");
  assert_eq!(order("test10 test1 test-10"), "test-10 test1 test10");
  assert_eq!(order(""), "");
}

