// https://www.codewars.com/kata/57c1ab3949324c321600013f

use std::collections::HashMap;

fn to_leet_speak(s: &str) -> String {
    let leet_map: HashMap<char, &str> = vec![
        ('A', "@"),
        ('B', "8"),
        ('C', "("),
        ('D', "D"),
        ('E', "3"),
        ('F', "F"),
        ('G', "6"),
        ('H', "#"),
        ('I', "!"),
        ('J', "J"),
        ('K', "K"),
        ('L', "1"),
        ('M', "M"),
        ('N', "N"),
        ('O', "0"),
        ('P', "P"),
        ('Q', "Q"),
        ('R', "R"),
        ('S', "$"),
        ('T', "7"),
        ('U', "U"),
        ('V', "V"),
        ('W', "W"),
        ('X', "X"),
        ('Y', "Y"),
        ('Z', "2"),
        (' ', " ")
    ]
    .into_iter()
    .collect();

    s.chars().map(|c| *leet_map.get(&c).unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_leet_speak() {
        assert_eq!(to_leet_speak("LEET"), "1337");
        assert_eq!(to_leet_speak("CODEWARS"), "(0D3W@R$");
        assert_eq!(to_leet_speak("LOREM IPSUM DOLOR SIT AMET"), "10R3M !P$UM D010R $!7 @M37");
    }
}
