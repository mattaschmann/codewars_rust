// https://www.codewars.com/kata/530265044b7e23379d00076a

type Point = (f32, f32);

// ref: https://rosettacode.org/wiki/Ray-casting_algorithm
fn point_in_poly(poly: &[Point], point: Point) -> bool {
  println!("{:?}", poly);
  println!("{:?}", point);

  // need to pop the first onto the end
  let looped_poly = [poly, &[*poly.first().unwrap()]].concat();

  looped_poly.windows(2)
    .fold(false, |inside, x| {
      println!("inside {:?}", inside);
      println!("x {:?}", x);
      if let [a, b] = x {
        match point_intersects(point, a, b) {
          true => return !inside,
          false => return inside
        };
      }
      unreachable!();
    })
}

fn point_intersects(p: Point, a: &Point, b: &Point) -> bool {
  // need to know which point is higher and which is lower
  let (lower, higher) = match a.1 < b.1 {
    true => (a, b),
    false => (b, a)
  };

  println!("l h p {:?} {:?} {:?}", lower, higher, p);

  // check the simple cases
  // is it between y's?
  if p.1 < lower.1 || p.1 > higher.1 { return false; }
  // is it right of the x's
  if p.0 > a.0.max(b.0) { return false; }
  // is it left of the x's
  if p.0 < a.0.min(b.0) { return true; }

  // check the slopes
  let edge_slope = (higher.1 - lower.1)/(higher.0 - lower.0);
  let p_slope = (p.1 - lower.1)/(p.0 - lower.0);

  println!("slopes {:?} {:?}", p_slope, edge_slope);
  println!("slope greater {:?}", p_slope > edge_slope);

  p_slope >= edge_slope
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn intersects() {
    assert_eq!(point_intersects((0.0, 0.0), &(1.0, 1.0), &(1.0, -1.0)), true);
    assert_eq!(point_intersects((0.0, 0.0), &(-1.0, 1.0), &(-1.0, -1.0)), false);
    assert_eq!(point_intersects((0.0, 0.0), &(1.0, 1.0), &(0.0, -2.0)), true);
    assert_eq!(point_intersects((0.0, 0.0), &(1.0, 1.0), &(0.0, -2.0)), true);
  }

  #[test]
  fn simple_square() {
    let poly = [(-5., -5.), (5., -5.), (5., 5.), (-5., 5.)];
    assert_eq!(point_in_poly(&poly, (-6., 0.)), false);
    assert_eq!(point_in_poly(&poly, (-1., 1.)), true);
  }

  #[test]
  fn simple_triangle() {
    let poly = [(-5.0, -5.0), (5.0, -5.0), (0.0, 5.0)];
    // assert_eq!(point_in_poly(&poly, (-4.0, -4.0)), true);
    assert_eq!(point_in_poly(&poly, (1.0, 3.0)), true);
  }

}
