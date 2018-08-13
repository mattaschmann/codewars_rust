// https://www.codewars.com/kata/diophantine-equation

fn solequa(n: u64) -> Vec<(u64, u64)> {

  (1..(n as f64).sqrt().ceil() as u64 + 1)
    .filter(|i| n % i == 0)
    .fold(Vec::new(), |mut acc, i| {
      let j = n/i;
      if i > j { return acc }

      let y = (j-i)/4;
      let x = i+2*y;

      if j == x+2*y && i == x-2*y { acc.push((x, y)) }

      acc
    })
}


#[cfg(test)]
mod tests {
  use super::*;

  fn testing(n: u64, exp: Vec<(u64, u64)>) -> () {
    println!("n = {}", n);
    assert_eq!(solequa(n), exp)
  }

  #[test]
  fn test_solequa() {
    testing(5, vec![(3, 1)]);
    testing(7, vec![]);
    testing(9, vec![(5, 2), (3, 0)]);
    testing(20, vec![(6, 2)]);
    testing(9001, vec![(4501, 2250)]);
    testing(9004, vec![(2252, 1125)]);
    testing(9009, vec![(4505, 2252), (1503, 750), (647, 320), (505, 248), (415, 202), (353, 170), (225, 102), (153, 60), (135, 48), (103, 20), (97, 10), (95, 2)]);
    testing(90005, vec![(45003, 22501), (9003, 4499), (981, 467), (309, 37)]);
    testing(90002, vec![]);
    testing(9000001, vec![(4500001, 2250000), (73801, 36870)]);
  }

}
