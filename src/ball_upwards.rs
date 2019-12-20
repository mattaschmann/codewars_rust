// https://www.codewars.com/kata/ball-upwards

const G: f64 = 0.981;

fn max_ball(v0: i32) -> i32 {
  let vel = v0 as f64 * 1000. / 60. / 60.;
  let mut time = 0;
  let mut height = 0.;
  let mut delta = 0.;

  let new_height = |v, t| v * t as f64 - 0.5 * G * (t * t) as f64;

  while delta >= 0. {
    time += 1;
    let new = new_height(vel, time);
    delta = new - height;
    height = new;
  }

  time - 1
}

#[cfg(test)]
mod tests {
  use super::*;

  fn testequal(v0: i32, exp: i32) -> () {
    assert_eq!(exp, max_ball(v0))
  }

  #[test]
  fn basic_tests() {
    testequal(37, 10);
    testequal(45, 13);
  }
}
