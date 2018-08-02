const NUMERALS: [(usize, &str, &str); 4] = [
  (1000, "M", "n"),
  (100,  "C", "D"),
  (10,   "X", "L"),
  (1,    "I", "V"),
];

/// Converts a number to a string representating roman numeral.
fn num_as_roman(num: i32) -> String {
  let mut numeral = String::new();
  let mut rem = num as usize;
  let mut ten = "n";

  for &(div, one, five) in NUMERALS.iter() {
    let count = rem / div;

    numeral += &one.repeat(count)
                   .replace(&one.repeat(9), &format!("{}{}", one , ten))
                   .replace(&one.repeat(5), five)
                   .replace(&one.repeat(4), &format!("{}{}", one , five));

    ten = one;
    rem = rem % div;
  }

  numeral
}

#[test]
fn returns_expected() {
  assert_eq!(num_as_roman(1),    "I");
  assert_eq!(num_as_roman(4),    "IV");
  assert_eq!(num_as_roman(5),    "V");
  assert_eq!(num_as_roman(6),    "VI");
  assert_eq!(num_as_roman(9),    "IX");
  assert_eq!(num_as_roman(182), "CLXXXII");
  assert_eq!(num_as_roman(1990), "MCMXC");
  assert_eq!(num_as_roman(1875), "MDCCCLXXV");
}
