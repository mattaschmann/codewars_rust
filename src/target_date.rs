// https://www.codewars.com/kata/569218bc919ccba77000000b

extern crate chrono;

use self::chrono::{NaiveDate, Duration};

fn date_nb_days(a0: i32, a: i32, p: i32) -> String {
    let mut current: f64 = a0 as f64;
    let mut days = 0;

    // This should be calculated using math
    while current < a as f64 {
      current += current * p as f64 / 36000.0;
      days += 1;
    }

    let start_date = NaiveDate::from_ymd(2016, 1, 1);
    let end_date = start_date.checked_add_signed(Duration::days(days));

    end_date.unwrap().format("%Y-%m-%d").to_string()
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(a0: i32, a: i32, p: i32, exp: &str) -> () {
        println!(" a0: {:?};", a0);
        println!("a: {:?};", a);
        println!("p: {:?};", p);
        let ans = date_nb_days(a0, a, p);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(100, 150, 2, "2035-12-26");
        dotest(4281, 5087, 2, "2024-07-03");
        dotest(4620, 5188, 2, "2021-09-19");
        dotest(258, 625, 1, "2103-03-19");
    }
}
