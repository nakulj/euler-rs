fn lcm(n: u64) -> u64 {
  let mut values: Vec<u64> = (2..=n).collect();
  let mut lcm = 1;
  while !values.is_empty() {
    let factor = values[0];
    lcm *= factor;
    values = values
      .into_iter()
      .map(|x| if x % factor == 0 { x / factor } else { x })
      .filter(|&x| x > 1)
      .collect();
  }
  return lcm;
}

#[test]
fn test_lcm() {
  assert_eq!(lcm(10), 2520);
}

fn main() {
  println!("{}", lcm(20));
}
