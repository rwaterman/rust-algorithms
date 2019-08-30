pub fn gcd(p: u64, q: u64) -> u64 {
  if q == 0 {
    return p;
  }
  let r = p % q;
  return gcd(q, r);
}

fn factorial(n: u64) -> u64 {
  println!("start");
  if n == 0 {
    1
  } else {
    n * factorial(n - 1)
  }
}

#[cfg(test)]
mod tests {
  use crate::math::discrete::gcd;

  #[test]
  fn test_gcd_composite() {
    assert_eq!(gcd(10, 15), 5);
  }

  #[test]
  fn test_gcd_prime() {
    assert_eq!(gcd(13, 17), 1);
  }

  #[test]
  fn test_gcd_prime_and_composite() {
    assert_eq!(gcd(5, 17), 1);
  }
}
