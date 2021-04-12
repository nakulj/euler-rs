use num::integer::Integer;
fn nth_prime(n: u64) -> u64 {
  let mut capacity = 2;
  let mut primes = vec![false; capacity];
  while primes.iter().filter(|&p| *p).count() < n as usize {
    primes.resize(2 * capacity, true);
    for i in 0..primes.len() {
      if !primes[i] {
        continue;
      };
      let start = if i < capacity {
        capacity.div_ceil(&i) * i
      } else {
        2 * i
      };
      for j in (start..primes.len()).step_by(i) {
        primes[j] = false;
      }
    }
    capacity *= 2
  }
  return primes
    .into_iter()
    .enumerate()
    .filter(|(_, p)| *p)
    .map(|(i, _)| (i as u64))
    .nth((n - 1) as usize)
    .expect("Panic!");
}

#[test]
fn test_nth_prime() {
  assert_eq!(nth_prime(6), 13)
}
fn main() {
  println!("{}", nth_prime(10_001))
}
