struct Fib {
  f0: i32,
  f1: i32,
}

impl Fib {
  fn new() -> Fib {
    Fib { f0: 0, f1: 1 }
  }
}

impl Iterator for Fib {
  type Item = i32;
  fn next(&mut self) -> Option<Self::Item> {
    self.f1 = self.f0 + self.f1;
    self.f0 = self.f1 - self.f0;
    Some(self.f0)
  }
}

fn main() {
  println!(
    "{}",
    Fib::new()
      .take_while(|&i| i <= 4000000)
      .filter(|&i| i % 2 == 0)
      .sum::<i32>()
  );
}
