// https://www.codewars.com/kata/scheduling-shortest-job-first-or-sjf

fn sjf(jobs: &[usize], index: usize) -> usize {
  let cur = jobs[index];

  jobs
    .iter()
    .enumerate()
    .filter(|&(i, x)| x < &cur || (i <= index && x == &cur))
    .map(|(_, x)| x)
    .sum()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sjf_simple() {
    assert_eq!(sjf(&[100], 0), 100);
    assert_eq!(sjf(&[1], 0), 1);
  }

  #[test]
  fn test_sjf_multiple() {
    assert_eq!(sjf(&[3,1,2], 0), 6);
  }

  #[test]
  fn test_sjf_fifo() {
    assert_eq!(sjf(&[1,4,4,4], 2), 9);
    assert_eq!(sjf(&[8,1,4,4,1,2,7], 3), 12);
  }

}
