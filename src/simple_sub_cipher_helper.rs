// https://www.codewars.com/kata/simple-substitution-cipher-helper

use std::collections::HashMap;

struct Cipher {
  encode_map: HashMap<char, char>,
  decode_map: HashMap<char, char>,
}

impl Cipher {
  fn new(map1: &str, map2: &str) -> Cipher {
    let mut encode_map = HashMap::new();
    let mut decode_map = HashMap::new();

    for (e, d) in map1.chars().zip(map2.chars()) {
      encode_map.insert(e, d);
      decode_map.insert(d, e);
    }

    Cipher {
      encode_map,
      decode_map,
    }
  }

  fn encode(&self, string: &str) -> String {
    string
      .chars()
      .map(|c| *self.encode_map.get(&c).unwrap_or(&c))
      .collect()
  }
  fn decode(&self, string: &str) -> String {
    string
      .chars()
      .map(|c| *self.decode_map.get(&c).unwrap_or(&c))
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // Rust test example:
  // TODO: replace with your own tests (TDD), these are just how-to examples.
  // See: https://doc.rust-lang.org/book/testing.html

  #[test]
  fn examples() {
    let map1 = "abcdefghijklmnopqrstuvwxyz";
    let map2 = "etaoinshrdlucmfwypvbgkjqxz";

    let cipher = Cipher::new(map1, map2);

    assert_eq!(cipher.encode("abc"), "eta");
    assert_eq!(cipher.encode("xyz"), "qxz");
    assert_eq!(cipher.decode("eirfg"), "aeiou");
    assert_eq!(cipher.decode("erlang"), "aikcfu");
  }
}
