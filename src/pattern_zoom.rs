fn zoom(n: i32) -> String {
  if n == 1 { return "■".to_string(); }

  let mut pattern_vec = Vec::new();

  pattern_vec.push(gen_str(n));

  for l in zoom(n - 2).split('\n') {
    let mut line = String::new();
    line.push(ring_box(n));
    line += l;
    line.push(ring_box(n));
    pattern_vec.push(line)
  }

  pattern_vec.push(gen_str(n));

  pattern_vec.join("\n")
}

fn ring_box(n: i32) -> char {
  let odd_box = '□';
  let even_box = '■';

  match ((n - 1) / 2) % 2 {
    0 => even_box,
    1 => odd_box,
    _ => '0'
  }
}

fn gen_str(n: i32) -> String {
  (0..n).map(|_| ring_box(n)).collect::<String>()
}


#[test]
fn test_zoom() {
  assert_eq!(zoom(1), "■");
  assert_eq!(zoom(3), "\
□□□
□■□
□□□");
  assert_eq!(zoom(5), "\
■■■■■
■□□□■
■□■□■
■□□□■
■■■■■");
}

#[test]
fn test_ring_box() {
  assert_eq!(ring_box(1), '■');
  assert_eq!(ring_box(3), '□');
}

#[test]
fn test_gen_str() {
  assert_eq!(gen_str(1), "■");
  assert_eq!(gen_str(3), "□□□");
}
