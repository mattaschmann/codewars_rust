fn interpreter(tape: &str, data: &str) -> String {
  let mut tape_it = tape.chars().cycle();

  data.chars().fold(String::new(), |mut acc, mut c| {
    while tape_it.next().unwrap() != '0' {
      c = match c { '0' => '1', _ => '0' };
    }
    acc.push(c);
    acc
  })
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_interpreter() {
    assert_eq!(interpreter("10", "10"), "01");
    assert_eq!(interpreter("10", "01"), "10");
    // flip every bit
    assert_eq!(interpreter("10", "1010101"), "0101010");
    // flip every other bit
    assert_eq!(interpreter("100", "1111111111"), "0101010101");
  }
}
