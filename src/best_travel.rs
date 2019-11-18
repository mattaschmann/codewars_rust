// https://www.codewars.com/kata/55e7280b40e1c4a06d0000aa

extern crate itertools;
use self::itertools::Itertools;

fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    let mut largest = -1;
    for cur in ls.iter().combinations(k as usize) {
        let sum = cur.iter().map(|x| *x).sum();
        if sum == t {
            return sum;
        };
        if sum > largest && sum < t {
            largest = sum
        };
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(t: i32, k: i32, ls: &Vec<i32>, exp: i32) -> () {
        assert_eq!(choose_best_sum(t, k, ls), exp)
    }

    #[test]
    fn basics_choose_best_sum() {
        let ts = &vec![50, 55, 56, 57, 58];
        testing(163, 3, ts, 163);
        let ts = &vec![50];
        testing(163, 3, ts, -1);
        let ts = &vec![91, 74, 73, 85, 73, 81, 87];
        testing(230, 3, ts, 228);
        testing(331, 2, ts, 178);
    }
}
