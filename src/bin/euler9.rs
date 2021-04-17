fn is_pythagorean(a: u64, b: u64, c: u64) -> bool {
  a * a + b * b == c * c
}

#[test]
fn test_is_pythagorean() {
  assert!(is_pythagorean(3, 4, 5));
  assert!(!is_pythagorean(3, 4, 7));
}

fn main() {
  let n = 1000;
  for i in 1..(n - 2) {
    for j in i..(n - i - 1) {
      let k = n - i - j;
      if is_pythagorean(i, j, k) {
        println!("{}", i * j * k);
      }
    }
  }
}
