// https://www.codewars.com/kata/diophantine-equation

fn solequa(n: u64) -> Vec<(u64, u64)> {
  println!("n: {}", n);
  let mut res = Vec::new();;
  let mut i = match n % 2 {
    0 => n + 1,
    _ => n
  };

  while i > 1 {
    let x = (i as f64 / 2_f64).ceil();
    let y = (i as f64 / 4_f64).floor();

    // break early if there are no other possible solutions
    if (x as u64).pow(2) < n { break }

    for j in (0..(y as u64)+1).rev() {
      let z = (x as u64).pow(2) - 4*(j).pow(2);
      if z == n {
        res.push((x as u64, j));
        break;
      }
      if z > n { break }
    }

    if i > 1 { i -= 2 }
  }

  res
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
    testing(9000001, vec![]);
  }
}
