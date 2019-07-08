pub fn gcd(p: u64, q: u64) -> u64 {
  if q == 0 {
    return p;
  }
  let r = p % q;
  return gcd(q, r);
}

pub fn max(slice: &[i64]) -> i64 {
  let mut max = slice[0];

  for &var in slice {
    if var > max {
      max = var;
    }
  }

  return max;
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

  use crate::math::discrete::max;

  #[test]
  fn test_max_postive() {
    assert_eq!(max(&[5,50,3]), 50);
  }

  #[test]
  fn test_max_postive_negative() {
    assert_eq!(max(&[5,-50,3]), 5);
  }

  #[test]
  fn test_max_negative_negative() {
    assert_eq!(max(&[-5,-50,-3]), -5);
  }
}
