// https://www.codewars.com/kata/554a44516729e4d80b000012

fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
  let mut assets = old as f64;
  let mut saved = 0f64;
  let mut cost = new as f64;
  let mut inflation = perc;
  let mut months = 0;

  while cost > (assets + saved) {
    months += 1;

    if months % 2 == 0 {
      inflation += 0.5;
    }

    assets -= assets * (inflation / 100.0);
    cost -= cost * (inflation / 100.0);
    saved += saving as f64;

  }

  (months, (assets + saved - cost).round() as i32)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn testing(old: i32, new: i32, saving: i32, perc: f64, exp: (i32, i32)) -> () {
    assert_eq!(nb_months(old, new, saving, perc), exp)
  }

  #[test]
  fn basics_nb_months() {
    testing(12000, 8000, 1000, 1.5, (0, 4000));
    testing(2000, 8000, 1000, 1.5, (6, 766));
    testing(8000, 12000, 500, 1.0, (8, 597));
    testing(18000, 32000, 1500, 1.25, (8, 332));
    testing(7500, 32000, 300, 1.55, (25, 122));
  }
}
