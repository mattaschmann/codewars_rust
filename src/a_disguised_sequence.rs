// https://www.codewars.com/kata/a-disguised-sequence-i

// After reducing on paper, this is just 2^n
fn fcn(n: i32) -> i64 {
  2_i64.pow(n as u32)
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_fcn() {
    assert_eq!(fcn(2), 4);
    assert_eq!(fcn(8), 2_i32.pow(8).into());
  }
}
