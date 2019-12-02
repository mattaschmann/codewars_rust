// https://www.codewars.com/kata/546d5028ddbcbd4b8d001254

use std::collections::HashMap;

fn partitions(n: isize) -> isize {
    let mut map = HashMap::new();

    part_helper(n, n, &mut map)
}

fn part_helper(n: isize, largest: isize, map: &mut HashMap<(isize, isize), isize>) -> isize {
    if largest == 0 { return 0; }
    if n == 0 { return 1; }
    if n < 0 { return 0; }

    let key = &(n, largest);

    match map.contains_key(key) {
        true => *map.get(key).unwrap(),
        false => {
            let res = part_helper(n, largest-1, map) + part_helper(n-largest, largest, map);
            map.insert(*key, res);

            res
        }
    }
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

    // #[test]
    // fn basic_test_25() {
    //     assert_eq!(partitions(25), 1958);
    // }

    // #[test]
    // fn basic_test_50() {
    //     assert_eq!(partitions(50), 204226);
    // }

    // #[test]
    // fn basic_test_75() {
    //     assert_eq!(partitions(75), 204226);
    // }

    #[test]
    fn basic_test_100() {
        assert_eq!(partitions(100), 1958);
    }

}
