use std::collections::HashMap;

fn factorial(n: u64, map: &HashMap<u64, u64>) -> u64 {
  if n < 2 { return 1 }

  match map.get(&n) {
    Some(x) => x.clone(),
    _ => {
      n * factorial(n-1, map)
    }
  }
}

// fn dec2_fact_string(nb: u64) -> String {
//   // your code
// }

// fn fact_string_2dec(s: String) -> u64 {
//   // your code
// }


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_factorial() {
    let map = HashMap::new();
    assert_eq!(factorial(0, &map), 1);
    assert_eq!(factorial(1, &map), 1);
    assert_eq!(factorial(2, &map), 2);
    assert_eq!(factorial(6, &map), 720);
  }
}
