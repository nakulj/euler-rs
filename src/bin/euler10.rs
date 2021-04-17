use euler_rs::primes::sift;
fn main() {
  println!("{}", sift(2_000_000).into_iter().sum::<u64>());
}
