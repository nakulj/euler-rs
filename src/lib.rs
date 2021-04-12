pub mod primes {
  pub fn sift(n: u64) -> Vec<u64> {
    let mut primes = vec![true; n as usize];
    primes[0] = false;
    primes[1] = false;
    for i in 0..primes.len() {
      if !primes[i] {
        continue;
      };
      for j in ((2 * i)..primes.len()).step_by(i) {
        primes[j] = false;
      }
    }
    return primes
      .into_iter()
      .enumerate()
      .filter(|(_, p)| *p)
      .map(|(i, _)| (i as u64))
      .collect();
      }

  pub fn largest_prime_factor(n: u64) -> u64 {
    sift((n as f64).sqrt() as u64 + 1)
      .into_iter()
      .rev()
      .find(|v| n % v == 0)
      .expect("Panic!")
  }
  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_sift() {
      assert_eq!(sift(5), vec![2, 3]);
      assert_eq!(sift(14), vec![2, 3, 5, 7, 11, 13]);
    }
    #[test]
    fn test_larget_prime_factor() {
      assert_eq!(largest_prime_factor(4), 2);
      assert_eq!(largest_prime_factor(13195), 29);
    }
  }
}
