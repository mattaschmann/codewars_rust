// https://www.codewars.com/kata/range-extraction/train/rust

mod solution {

  pub fn range_extraction(a: &[i32]) -> String {
    a.windows(2)
      .fold(vec![vec![a[0]]], |mut acc: Vec<Vec<i32>>, win| {
        println!("{:?}", win);

        if win[1] - win[0] > 1 {
          acc.push(vec![win[1]]);
        } else {
          let mut last = acc.pop().unwrap();
          last.push(win[1]);
          acc.push(last);
        }

        println!("{:?}", acc);
        acc
      })
      .iter()
      .map(|range| match range.len() {
        1 => range[0].to_string(),
        2 => format!("{},{}", range[0], range[1]),
        _ => format!("{}-{}", range[0], range.last().unwrap()),
      })
      .collect::<Vec<String>>()
      .join(",")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example() {
    assert_eq!(
      "-6,-3-1,3-5,7-11,14,15,17-20",
      solution::range_extraction(&[
        -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
      ])
    );
    assert_eq!(
      "-3--1,2,10,15,16,18-20",
      solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20])
    );
  }
}
