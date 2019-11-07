// https://www.codewars.com/kata/54b42f9314d9229fd6000d9c

fn duplicate_encode(word: &str) -> String {
  let mut char_map = std::collections::HashMap::new();
  let word = word.to_lowercase();

  for c in word.chars() {
    let cur = char_map.entry(c).or_insert(0);
    *cur += 1;
  }

  println!("{:?}", char_map);

  word.chars().map(|c| {
    println!("{}", c);
    match char_map.get(&c).unwrap() > &1 {
      true => ')',
      false => '('
    }
  }).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn run_tests() {
    assert_eq!(duplicate_encode("din"), "(((");
    assert_eq!(duplicate_encode("recede"), "()()()");
    assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
    assert_eq!(duplicate_encode("(( @"), "))((");
  }
}
