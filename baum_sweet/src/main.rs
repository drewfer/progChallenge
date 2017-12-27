use std::env::args;

fn baum_sweet(n : u64) -> u8 {
  let mut bits = n;
  loop {
    if bits == 0 { return 1 }
    let n = bits.trailing_zeros();
    if n % 2 == 1 { return 0 }  
    bits >>= n + 1;
  }
}


#[cfg(not(test))]
fn main() {
  let mut argv = args();
  if let Some(arg1) = argv.nth(1) {
    if let Ok(num) = arg1.parse::<u64>() {
      print!("{}", baum_sweet(0));
      for i in 1..num {
        print!(", {}", baum_sweet(i));
      }
      return
    }
  }
  print!("Usage: baum_sweet <int>");
}


#[cfg(test)]
mod test {
  use super::baum_sweet;

  #[test]
  fn test_first_few() {
    let known : [u8;16] = [1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1];
    for i in 0..known.len() {
      assert!(known[i] == baum_sweet(i as u64));
    }
  }

  #[test]
  fn test_large() {
    assert!(0 == baum_sweet(19611206u64));
  }
}
