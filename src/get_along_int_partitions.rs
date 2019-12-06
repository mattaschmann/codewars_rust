// https://www.codewars.com/kata/getting-along-with-integer-partitions

use std::collections::HashSet;

type Combo = Vec<i64>;
type Combos = Vec<Combo>;

fn part(n: i64) -> String {
  let combos = get_combos(n);
  let prod = gen_prod(combos);

  println!("{:?}", prod);

  let max = prod.last().unwrap();
  let sum: i64 = prod.iter().sum();
  let halfway = prod.len() / 2;

  let range = max - 1;
  let average = sum as f64 / prod.len() as f64;
  let median = match prod.len() % 2 {
    0 => (prod[halfway - 1] + prod[halfway]) as f64 / 2.,
    1 => prod[halfway] as f64,
    _ => unreachable!()
  };

  format!("Range: {} Average: {:.2} Median: {:.2}", range, average, median)
}

fn get_combos(n: i64) -> Combos {
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

fn gen_prod(combos: Combos) -> Vec<i64> {
  let mut set = HashSet::new();
  let mut prod = Vec::new();

  for c in combos.iter() {
    let product = c.iter().take_while(|n| n != &&1).fold(1, |acc, n| n * acc);
    if set.insert(product) { prod.push(product); }
  }

  prod.sort();

  prod
}

#[cfg(test)]
mod tests {
  use super::*;

  fn testequal(ans: &str, sol: &str) {
    assert!(ans == sol, "Expected \"{}\", got \"{}\".", sol, ans);
  }

  #[test]
  fn test_combos() {
    assert_eq!(
      get_combos(5),
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

  #[test]
  fn test_gen_prod() {
    assert_eq!(
      gen_prod(vec![
        vec![5],
        vec![4, 1],
        vec![3, 2],
        vec![3, 1, 1],
        vec![2, 2, 1],
        vec![2, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
      ]),
      vec![1, 2, 3, 4, 5, 6]
    )
  }

  #[test]
  fn returns_expected() {
    testequal(&part(1), "Range: 0 Average: 1.00 Median: 1.00");
    testequal(&part(2), "Range: 1 Average: 1.50 Median: 1.50");
    testequal(&part(3), "Range: 2 Average: 2.00 Median: 2.00");
    testequal(&part(4), "Range: 3 Average: 2.50 Median: 2.50");
    testequal(&part(5), "Range: 5 Average: 3.50 Median: 3.50");
  }

}
