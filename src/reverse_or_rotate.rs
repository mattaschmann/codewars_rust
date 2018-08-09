fn revrot(s: &str, c: usize) -> String {
  if s.len() < c || s.is_empty() || c == 0 { return "".to_string() }

  let (cur, rest) = s.split_at(c);
  let sum: u32 = cur.chars().map(|c| c.to_digit(10).unwrap().pow(3)).sum();

  let mut result;
  if sum % 2 == 0 {
    result = cur.chars().rev().collect();
  } else {
    result = String::from(&cur[1..]);
    result.push(cur.chars().next().unwrap());
  }

  result + &revrot(rest, c)
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_revrot() {
    assert_eq!(revrot("1234", 0), "");
    assert_eq!(revrot("", 5), "");
    assert_eq!(revrot("1234", 5), "");
    assert_eq!(revrot("1234", 2), "2143");
    assert_eq!(revrot("131246", 3), "311642");
  }

  fn testing(s: &str, c: usize, exp: &str) -> () {
    assert_eq!(&revrot(s, c), exp)
  }

  #[test]
  fn basics_revrot() {
    testing("1234", 0, "");
    testing("", 0, "");
    testing("1234", 5, "");
    let s = "733049910872815764";
    testing(s, 5, "330479108928157");
    let s = "73304991087281576455176044327690580265896";
    testing(s, 8, "1994033775182780067155464327690480265895");
  }
}
