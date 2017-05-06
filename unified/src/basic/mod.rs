


pub fn hamming(a: &str, b: &str) -> i32 {
  let mut mismatches = 0;

  let a_chars: Vec<char> = a.chars().collect();
  let b_chars: Vec<char> = b.chars().collect();

  for i in 0..a_chars.len() as usize {
    if a_chars[i] != b_chars[i] { mismatches += 1; }
  }

  mismatches
}
