// https://www.codewars.com/kata/55466989aeecab5aac00003e

use std::cmp;

fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    if lng == wdth { return None; }

    let large = cmp::max(lng, wdth);
    let small = cmp::min(lng, wdth);

    match sq_in_rect(small, large - small) {
        None => Some(vec![small, small]),
        Some(x) => Some([vec![small], x].concat())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(lng: i32, wdth: i32, exp: Option<Vec<i32>>) -> () {
        assert_eq!(sq_in_rect(lng, wdth), exp)
    }

    #[test]
    fn tests_sq_in_rect() {
        testing(5, 3, Some(vec![3, 2, 1, 1]));
        testing(3, 5, Some(vec![3, 2, 1, 1]));
        testing(5, 5, None);
    }
}
