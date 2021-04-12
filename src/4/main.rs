use itertools::Itertools;
fn is_palindrome(n: u64) -> bool {
  let s = format!("{}", n);
  return s == s.chars().rev().collect::<String>();
}

#[test]
fn test_is_palindrome() {
  assert!(is_palindrome(1));
  assert!(!is_palindrome(21));
}

fn largest_palindrome_product(d: u32) -> u64 {
  let r1 = 10u64.pow(d - 1)..10u64.pow(d);
  let r2 = r1.clone();
  r1.cartesian_product(r2)
    .map(|(i, j)| i * j)
    .filter(|&x| is_palindrome(x))
    .fold(0, |a, b| a.max(b))
}

#[test]
fn test_largest_palindrome_product() {
  assert_eq!(largest_palindrome_product(2), 9009);
}

fn main() {
  println!("{}", largest_palindrome_product(3));
}
