// https://www.codewars.com/kata/array-dot-diff

fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
  a.into_iter()
   .filter(|x| !b.contains(&x))
   .collect()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_array_diff() {
    assert_eq!(array_diff(vec![1,2], vec![1]), vec![2]);
    assert_eq!(array_diff(vec![1,2,2], vec![1]), vec![2,2]);
    assert_eq!(array_diff(vec![1,2,2], vec![2]), vec![1]);
    assert_eq!(array_diff(vec![1,2,2], vec![]), vec![1,2,2]);
    assert_eq!(array_diff(vec![], vec![1,2]), vec![]);
  }
}
