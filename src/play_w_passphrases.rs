// https://www.codewars.com/kata/559536379512a64472000053

fn play_pass(s: &str, n: u32) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if c.is_alphabetic() {
                let mut ascii = c as u8;
                ascii += n as u8;
                let mut shifted = ascii as char;

                if !shifted.is_alphabetic() {
                    ascii -= 26;
                    shifted = ascii as char;
                }
                match i % 2 == 1 {
                    true => return shifted.to_ascii_lowercase(),
                    false => return shifted.to_ascii_uppercase(),
                }
            } else if c.is_numeric() {
                return std::char::from_digit(9 - c.to_digit(10).unwrap(), 10).unwrap();
            }

            c
        })
        .collect::<String>()
        .chars()
        .rev()
        .collect()
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
    }
}
