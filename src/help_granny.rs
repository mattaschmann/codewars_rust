// https://www.codewars.com/kata/help-your-granny

use std::collections::HashMap;

fn tour(frnds: &[&str], fr_twns: HashMap<&str, &str>, dist: HashMap<&str, f64>) -> i32 {
  let distance = |a: &f64, c: &f64| (c*c - a*a).sqrt();

  let first_leg = dist.get(fr_twns.get(frnds[0]).unwrap()).unwrap();
  let res = frnds.iter().skip(1).fold((*first_leg, 0.), |(prev, sum): (f64, f64), f| {
    match fr_twns.get(f) {
      Some(t) => {
        let d = dist.get(t).unwrap();
        (*d, sum + distance(&prev, d))
      },
      None => (prev, sum)
    }
  });

  (first_leg + res.1.floor() + res.0) as i32
}

#[cfg(test)]
mod tests {
  use super::*;

  fn testing(
    frnds: &[&str],
    fr_twns: HashMap<&str, &str>,
    dist: HashMap<&str, f64>,
    exp: i32,
  ) -> () {
    assert_eq!(tour(&frnds, fr_twns, dist), exp)
  }

  #[test]
  fn tests_tour() {
    let friends = ["A1", "A2", "A3", "A4", "A5"];
    let fr_towns = hash_map! { "A1" => "X1", "A2"=> "X2", "A3" => "X3", "A4" => "X4" };
    let dst = hash_map! { "X1" => 100.0, "X2" => 200.0, "X3" => 250.0, "X4" => 300.0 };
    testing(&friends, fr_towns, dst, 889);
  }
}
