pub fn gcd(p: u64, q: u64) -> u64 {
  if q == 0 { return p; }
  let r = p % q;
  return gcd(q, r);
}
