use std::collections::HashMap;
use std::char;

const ASCII_ARRAY: [char; 36] = ['0','1','2','3','4','5','6','7','8','9',
                                 'A','B','C','D','E','F','G','H','I','J',
                                 'J','L','M','N','O','P','Q','R','S','T',
                                 'U','V','W','X','Y','Z'];

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
  println!("{}", nb);
  let map = HashMap::new();

  let d = largest_digit(nb, &map);
  let mut result = String::new();
  let mut rem = nb;
  for i in (0..d+1).rev() {
    let f = factorial(i, &map);
    println!("{}", rem / f);
    let c = ASCII_ARRAY[(rem / f) as usize];
    println!("{}", c);
    result.push(c);
    rem %= f;
  }

  result
}

fn fact_string_2dec(s: String) -> u64 {
  println!("{}", s);
  let map = HashMap::new();
  s.chars()
   .rev()
   .enumerate()
   .map(|(i, c)| factorial(i as u64, &map) * ASCII_ARRAY.iter().position(|&x| c == x).unwrap() as u64)
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
    assert_eq!(dec2_fact_string(36288000), "A0000000000");
  }

  #[test]
  fn test_fact_string_2dec() {
    assert_eq!(fact_string_2dec("4041000".to_string()), 2982);
    assert_eq!(fact_string_2dec("341010".to_string()), 463);
    assert_eq!(fact_string_2dec("A0000000000".to_string()), 36288000);
    assert_eq!(fact_string_2dec("27A0533231100".to_string()), 1273928000);
  }

}
