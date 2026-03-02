pub fn always_one() -> i32 {
  1
}

#[cfg(test)]
mod tests {
  use super::always_one;

  #[test]
  fn should_fail() {
    assert_eq!(always_one(), 2);
  }
}
