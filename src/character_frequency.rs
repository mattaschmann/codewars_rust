// https://www.codewars.com/kata/53e895e28f9e66a56900011a

use std::collections::BTreeMap;

fn letter_frequency(input: &str) -> BTreeMap<char, i32> {
  input
    .chars()
    .filter(|c| c.is_alphabetic())
    .fold(BTreeMap::new(), |mut acc, c| {
      *acc.entry(c.to_lowercase().next().unwrap()).or_insert(0) += 1;
      acc
    })
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[cfg(test)]
mod tests {
  use super::letter_frequency;
  use std::collections::BTreeMap;

  #[test]
  fn simpleword() {
    let answer: BTreeMap<char, i32> = [('a', 2), ('c', 1), ('l', 1), ('t', 1), ('u', 1)]
      .iter()
      .cloned()
      .collect();

    assert_eq!(letter_frequency("actual"), answer);
  }

  #[test]
  fn sequence() {
    let answer: BTreeMap<char, i32> = [
      ('a', 3),
      ('b', 2),
      ('f', 1),
      ('p', 1),
      ('s', 1),
      ('t', 2),
      ('u', 1),
      ('x', 5),
    ]
    .iter()
    .cloned()
    .collect();

    assert_eq!(letter_frequency("AaabBF UttsP xxxxx"), answer);
  }
}
