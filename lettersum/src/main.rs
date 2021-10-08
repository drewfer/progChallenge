fn lettersum(s: &str) -> u32 {
  s
   .bytes()
   .map(|b| (b - b'a' + 1) as u32)
   .sum()
}

#[cfg(not(test))]
fn main() {
  print!("lettersum(\"\") => {}\n", lettersum(""));
  print!("lettersum(\"a\") => {}\n", lettersum("a"));
  print!("lettersum(\"z\") => {}\n", lettersum("z"));
  print!("lettersum(\"cab\") => {}\n", lettersum("cab"));
  print!("lettersum(\"excellent\") => {}\n", lettersum("excellent"));
  print!("lettersum(\"microspectrophotometries\") => {}\n", lettersum("microspectrophotometries"));
}


#[cfg(test)]
mod test {
  use super::lettersum;

  #[test]
  fn test_blank() {
    assert!(0 == lettersum(""));
  }

  #[test]
  fn test_a() {
    assert!(1 == lettersum("a"));
  }

  #[test]
  fn test_z() {
    assert!(26 == lettersum("z"));
  }

  #[test]
  fn test_cab() {
    assert!(6 == lettersum("cab"));
  }

  #[test]
  fn test_excellent() {
    assert!(100 == lettersum("excellent"));
  }

  #[test]
  fn test_microspectrophotometries() {
    assert!(317 == lettersum("microspectrophotometries"));
  }
}