use std::collections::HashMap;
use std::char;

fn factorial(n: u64, map: &HashMap<u64, u64>) -> u64 {
  if n < 2 { return 1 }

  match map.get(&n) {
    Some(x) => x.clone(),
    _ => {
      n * factorial(n-1, map)
    }
  }
}

fn largest_digit(n: u64, map: &HashMap<u64, u64>) -> u64 {
  let mut d = 1;
  while factorial(d, map) < n { d += 1 }

  d-1
}

fn dec2_fact_string(nb: u64) -> String {
  let map = HashMap::new();

  let d = largest_digit(nb, &map);
  let mut result = String::new();
  let mut rem = nb;
  for i in (0..d+1).rev() {
    let f = factorial(i, &map);
    let c = char::from_digit((rem / f) as u32, 10).unwrap();
    result.push(c);
    rem %= f;
  }

  result
}

fn fact_string_2dec(s: String) -> u64 {
  let map = HashMap::new();
  s.chars()
   .rev()
   .enumerate()
   .map(|(i, c)| factorial(i as u64, &map) * c.to_digit(10).unwrap() as u64)
   .sum()
}


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

  #[test]
  fn test_largest_digit() {
    let map = HashMap::new();
    assert_eq!(largest_digit(730, &map), 6);
  }

  #[test]
  fn test_dec2_fact_string() {
    assert_eq!(dec2_fact_string(2982), "4041000");
    assert_eq!(dec2_fact_string(463), "341010");
  }

  #[test]
  fn test_fact_string_2dec() {
    assert_eq!(fact_string_2dec("4041000".to_string()), 2982);
    assert_eq!(fact_string_2dec("341010".to_string()), 463);
  }
}
