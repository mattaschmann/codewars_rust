// https://www.codewars.com/kata/54dc6f5a224c26032800005c

fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
  if list_art.is_empty() || list_cat.is_empty() {
    return "".to_string();
  }

  let mut catagory_map = std::collections::HashMap::new();

  for s in list_art {
    let mut s_iter = s.split_whitespace();

    let code = s_iter.next().unwrap();
    let count = s_iter.next().unwrap();

    let total = catagory_map.entry(code.get(0..1).unwrap()).or_insert(0);
    *total += count.parse::<u32>().unwrap();
  }

  list_cat
    .iter()
    .map(|c| format!("({} : {})", c, catagory_map.get(c).unwrap_or(&0)))
    .collect::<Vec<String>>()
    .join(" - ")
}

#[cfg(test)]
mod tests {
  use super::*;

  fn dotest(list_art: Vec<&str>, list_cat: Vec<&str>, exp: &str) -> () {
    println!("list_art: {:?};", list_art);
    println!("list_cat: {:?};", list_cat);
    let ans = stock_list(list_art, list_cat);
    println!("actual:\n{:?};", ans);
    println!("expect:\n{:?};", exp);
    println!("{};", ans == exp);
    assert_eq!(ans, exp);
    println!("{};", "-");
  }

  #[test]
  fn basic_tests() {
    let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
    let mut c = vec!["A", "B", "C", "D"];
    dotest(b, c, "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");

    b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
    c = vec!["A", "B"];
    dotest(b, c, "(A : 200) - (B : 1140)");
  }
}
