fn bouncing_ball(h: f64,  bounce: f64,  window: f64) -> i32 {
  // h must be greater than 0
  if !(h > 0.0) { return -1; }
  // bounce must be between 0 and 1
  if !(0.0 < bounce && bounce < 1.0) { return -1; }
  // window must be between 0 and h
  if !(0.0 < window && window < h) { return -1; }

  let mut count = 1;
  let mut rebound = h * bounce;
  while rebound > window {
    count += 2;
    rebound = rebound * bounce;
  }

  count
}

#[test]
fn tests_bouncing_ball() {
    assert_eq!(bouncing_ball(-1.0, 0.5, -2.0), -1, "h must be greater than 0");
    assert_eq!(bouncing_ball(1.0, -1.0, 0.5), -1, "bounce must be greater than 0");
    assert_eq!(bouncing_ball(1.0, 1.5, 0.5), -1, "bounce must be less than 1");
    assert_eq!(bouncing_ball(1.0, 0.5, 1.5), -1, "window must be less than h");
    assert_eq!(bouncing_ball(1.0, 0.5, -1.0), -1, "window must be greater than 0");
    assert_eq!(bouncing_ball(3.0, 0.66, 1.5), 3);
    assert_eq!(bouncing_ball(30.0, 0.66, 1.5), 15);
    assert_eq!(bouncing_ball(40.0, 0.4, 10.0), 3);
}
