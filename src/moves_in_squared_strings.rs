fn rot(s: &str) -> String {
  // your code
  s.chars().rev().collect()
}
fn add_dots(s: &str) -> String {
  s.split('\n')
   .map(|substr| substr.to_owned() + &".".repeat(substr.len()))
   .collect::<Vec<String>>()
   .join("\n")
}
fn selfie_and_rot(s: &str) -> String {
  // your code
  let with_dots = add_dots(s);
  let rot_with_dots = rot(&with_dots);
  with_dots + "\n" + &rot_with_dots
}
// first parameter: dots have to be replaced by function of one variable
fn oper(f: fn(&str) -> String, s: &str) -> String {
  f(s)
}


fn testing1(s: &str, exp: &str) -> () {
  assert_eq!(oper(rot, s), exp.to_string())
}
fn testing2(s: &str, exp: &str) -> () {
  assert_eq!(oper(selfie_and_rot, s), exp.to_string())
}

#[test]
fn basics_oper() {

  testing1("fijuoo\nCqYVct\nDrPmMJ\nerfpBA\nkWjFUG\nCVUfyL",
           "LyfUVC\nGUFjWk\nABpfre\nJMmPrD\ntcVYqC\nooujif");
  testing1("rkKv\ncofM\nzXkh\nflCB", "BClf\nhkXz\nMfoc\nvKkr");

  testing2("xZBV\njsbS\nJcpN\nfVnP",
           "xZBV....\njsbS....\nJcpN....\nfVnP....\n....PnVf\n....NpcJ\n....Sbsj\n....VBZx");
  testing2("uLcq\nJkuL\nYirX\nnwMB",
           "uLcq....\nJkuL....\nYirX....\nnwMB....\n....BMwn\n....XriY\n....LukJ\n....qcLu");

}

#[test]
fn test_rot() {
  assert_eq!(rot("abcd\nefgh\nijkl\nmnop"), "ponm\nlkji\nhgfe\ndcba")
}

#[test]
fn test_add_dots() {
  assert_eq!(add_dots("abcd\nefgh\nijkl\nmnop"), "abcd....\nefgh....\nijkl....\nmnop....")
}

#[test]
fn test_selfie_and_rot() {
  assert_eq!(selfie_and_rot("abcd\nefgh\nijkl\nmnop"), "abcd....\nefgh....\nijkl....\nmnop....\n....ponm\n....lkji\n....hgfe\n....dcba")
}
