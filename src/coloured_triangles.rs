// https://www.codewars.com/kata/5a25ac6ac5e284cfbe000111

fn triangle(row_str: &str) -> String {
  if row_str.len() <= 1 { return row_str.to_string() };

  let mut res = String::new();

  for i in 0..row_str.len()-1 {
    let snip = row_str.get(i..i+2);

    match snip {
      Some("GG") => res.push('G'),
      Some("RR") => res.push('R'),
      Some("BB") => res.push('B'),
      Some("BG") | Some("GB") => res.push('R'),
      Some("RG") | Some("GR") => res.push('B'),
      Some("BR") | Some("RB") => res.push('G'),
      _ => ()
    }

  }

  triangle(&res)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle() {
        assert_eq!(triangle("B"), "B");
        assert_eq!(triangle("GB"), "R");
        assert_eq!(triangle("RRR"), "R");
        assert_eq!(triangle("RGBG"), "B");
        assert_eq!(triangle("RBRGBRB"), "G");
        assert_eq!(triangle("RBRGBRBGGRRRBGBBBGG"), "G");
        assert_eq!(triangle("GB"), "R");
    }
}
