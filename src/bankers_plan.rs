// https://www.codewars.com/kata/56445c4755d0e45b8c00010a

fn fortune(f0: i32, p: f64, c0: i32, n: i32, i: f64) -> bool {
    let mut money: i64 = f0.into();
    let mut cost: i64 = c0.into();

    for _ in 0..n {
        money += (p / 100.0 * money as f64) as i64 - cost;

        if money <= 0 { return false; }
        cost += (i / 100.0 * cost as f64) as i64;
    }

    true
}


#[cfg(test)]
mod tests {
    use super::*;

    fn testequal(f0: i32, p: f64, c0: i32, n: i32, i: f64, exp: bool) -> () {
        assert_eq!(exp, fortune(f0, p, c0, n, i))
    }

    #[test]
    fn basics() {
        testequal(100000, 1.0, 2000, 15, 1.0, true);
        testequal(100000, 1.0, 9185, 12, 1.0, false);
        testequal(100000000, 1.0, 100000, 50, 1.0, true);
        testequal(100000000, 1.5, 10000000, 50, 1.0, false);
    }
}
