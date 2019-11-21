// https://www.codewars.com/kata/559536379512a64472000053

fn play_pass(s: &str, n: u32) -> String {
    s.char_indices()
        .map(|(i, c)| {
            if c.is_alphabetic() {
                // need to convert to lowercase so that we don't run into other alphanumeric chars
                let mut ascii = c.to_ascii_lowercase() as u8;
                ascii += n as u8;
                let mut shifted = ascii as char;

                if !shifted.is_alphabetic() {
                    ascii -= 26;
                    shifted = ascii as char;
                }
                match i % 2 == 1 {
                    true => return shifted,
                    false => return shifted.to_ascii_uppercase(),
                }
            } else if c.is_numeric() {
                return std::char::from_digit(9 - c.to_digit(10).unwrap(), 10).unwrap();
            }

            c
        })
        .rev()
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(s: &str, n: u32, exp: &str) -> () {
        println!(" s: {:?};", s);
        println!("n: {:?};", n);
        let ans = play_pass(s, n);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest("Zz", 1, "aA");
        dotest("1234", 1, "5678");
        dotest("I LOVE YOU!!!", 1, "!!!vPz fWpM J");
        dotest("I LOVE YOU!!!", 0, "!!!uOy eVoL I");
        dotest("AAABBCCY", 1, "zDdCcBbB");
        dotest("IN 2012 TWO CAMBRIDGE UNIVERSITY RESEARCHERS ANALYSED PASSPHRASES FROM THE AMAZON PAY SYSTEM...", 20, "...gYnMsM SuJ HiTuGu yBn gIlZ MyMuLbJmMuJ XyMsFuHu mLyBwLuYmYl sNcMlYpChO YaXcLvGuW IqN 7897 hC");
    }
}
