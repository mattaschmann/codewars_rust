// https://www.codewars.com/kata/546d5028ddbcbd4b8d001254

extern crate itertools;
use self::itertools::Itertools;

fn partitions(n: isize) -> isize {
    let mut combo = vec![n];
    let mut count = 1;

    while combo.first().unwrap() != &1 {
        while combo.last().unwrap() == &1 {
            combo.pop();
        }

        let mut temp = combo.pop().unwrap();
        temp -= 1;
        combo.push(temp);

        let mut rem: isize = n - combo.iter().sum::<isize>();

        println!("{}", rem);
        while rem > temp {
            combo.push(temp);
            rem -= temp;
        }

        combo.push(rem);
        count += 1;

        println!("{:?}", combo);
    }

    count
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
        assert_eq!(partitions(10), 42);
    }

    #[test]
    fn basic_test_25() {
        assert_eq!(partitions(25), 1958);
    }

    // #[test]
    // fn basic_test_100() {
    //     assert_eq!(partitions(100), 1958);
    // }

}
