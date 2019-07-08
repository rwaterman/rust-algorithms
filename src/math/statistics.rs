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
  use crate::math::statistics;

  #[test]
  fn test_max_postive() {
    assert_eq!(statistics::max(&[5,50,3]), 50);
  }

  #[test]
  fn test_max_postive_negative() {
    assert_eq!(statistics::max(&[5,-50,3]), 5);
  }

  #[test]
  fn test_max_negative_negative() {
    assert_eq!(statistics::max(&[-5,-50,-3]), -3);
  }
}