// https://www.codewars.com/kata/546d5028ddbcbd4b8d001254

extern crate itertools;
use self::itertools::Itertools;

fn partitions(n: isize) -> isize {
    if n == 1 { return 1; }

    let mut sum = 2;

    for i in 2..n {
        let combos = (1..n - (i - 2))
            .rev()
            .combinations_with_replacement(i as usize)
            .filter(|v| v.iter().sum::<isize>() == n)
            .collect::<Vec<Vec<isize>>>();

        println!("{:?}", combos);

        sum += combos.len() as isize;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_01() {
        assert_eq!(partitions(1), 1);
    }

    #[test]
    fn basic_test_05() {
        assert_eq!(partitions(5), 7);
    }

    #[test]
    fn basic_test_10() {
        assert_eq!(partitions(10), 41);
    }

    // #[test]
    // fn basic_test_25() {
    //     assert_eq!(partitions(25), 1958);
    // }

}
