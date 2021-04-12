fn diff(n: u64) -> u64 {
  (1..=n).sum::<u64>().pow(2) - (1..=n).map(|x| x * x).sum::<u64>()
}

#[test]
fn test_diff() {
  assert_eq!(diff(10), 2640);
}

fn main() {
  println!("{}", diff(100));
}
