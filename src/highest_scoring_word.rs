// https://www.codewars.com/kata/57eb8fcdf670e99d9b000272

fn high(input: &str) -> &str {
  input.split(' ').fold("", |acc: &str, w: &str| {
    match score_word(acc) < score_word(w) {
      true => w,
      _ => acc,
    }
  })
}

fn score_word(word: &str) -> u32 {
  word.chars().fold(0, |acc, c| acc + c as u32 - 96)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn test_score_word(word: &str, score: u32) -> () {
    time_test!(word);
    assert_eq!(score_word(word), score);
  }

  #[test]
  fn run_score_word() {
    test_score_word("a", 1);
    test_score_word("abcd", 10);
  }

  #[test]
  fn test_basic() {
    assert_eq!(high("man i need a taxi up to ubud"), "taxi");
    assert_eq!(high("what time are we climbing up the volcano"), "volcano");
    assert_eq!(high("take me to semynak"), "semynak");
    assert_eq!(high("massage yes massage yes massage"), "massage");
    assert_eq!(high("take two bintang and a dance please"), "bintang");
    assert_eq!(high("aaa b"), "aaa");
    assert_eq!(high(""), "");
  }
}
