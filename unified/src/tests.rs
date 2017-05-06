#[cfg(test)]

// use union_find::UnionFind;
use basic;

#[test]
// #[ignore]
fn hamming() {
  let mut ham = basic::hamming("xyz","pqr");
  assert!(ham == 3);

  ham = basic::hamming("xyz", "xyz");
  assert!(ham == 0);
}
