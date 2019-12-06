// https://www.codewars.com/kata/getting-along-with-integer-partitions

// fn part(n: i64) -> String {}

fn get_enum(n: i64) -> Vec<Vec<i64>> {
  let mut combo = vec![n];
  let mut combos = vec![combo.clone()];

  while combo.first().unwrap() != &1 {
    while combo.last().unwrap() == &1 {
      combo.pop();
    }

    let mut temp = combo.pop().unwrap();
    temp -= 1;
    combo.push(temp);

    let mut rem = n - combo.iter().sum::<i64>();

    while rem > temp {
      combo.push(temp);
      rem -= temp;
    }

    combo.push(rem);

    combos.push(combo.clone());
  }

  combos
}

#[cfg(test)]
mod tests {
  use super::*;

  fn testequal(ans: &str, sol: &str) {
    assert!(ans == sol, "Expected \"{}\", got \"{}\".", sol, ans);
  }

  #[test]
  fn test_enum() {
    assert_eq!(
      get_enum(5),
      vec![
        vec![5],
        vec![4, 1],
        vec![3, 2],
        vec![3, 1, 1],
        vec![2, 2, 1],
        vec![2, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
      ]
    )
  }

  // #[test]
  // fn returns_expected() {
  //   testequal(&part(1), "Range: 0 Average: 1.00 Median: 1.00");
  //   testequal(&part(2), "Range: 1 Average: 1.50 Median: 1.50");
  //   testequal(&part(3), "Range: 2 Average: 2.00 Median: 2.00");
  //   testequal(&part(4), "Range: 3 Average: 2.50 Median: 2.50");
  //   testequal(&part(5), "Range: 5 Average: 3.50 Median: 3.50");
  // }
}
