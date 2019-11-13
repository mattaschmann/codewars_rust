// https://www.codewars.com/kata/5839edaa6754d6fec10000a2

fn find_missing_letter(chars: &[char]) -> char {
    for x in chars.windows(2) {
        if let [cur, next] = x {
            let expected_digit = *cur as u8 + 1;
            if expected_digit != *next as u8 {
                return expected_digit as char;
            }
        }
    }

    unreachable!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
        assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
    }
}
