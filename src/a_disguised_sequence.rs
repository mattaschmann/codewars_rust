// https://www.codewars.com/kata/a-disguised-sequence-i

// After reducing on paper, this is just 2^n.  Shifting left is more efficient.
fn fcn(n: i32) -> i64 {
  1 << n
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
