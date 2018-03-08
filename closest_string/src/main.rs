use std::io::{stdin, BufRead};

fn hamming_distance(a : &str, b : &str) -> Result<usize, String> {
  if a.len() != b.len() {
     return Err("Strings of unequal length".to_string())
  }
  Ok(a.chars()
      .zip(b.chars())
      .filter(|&(ca, cb)| ca != cb)
      .count())
}

fn find_center(strs: &Vec<String>) -> Option<String> {
    strs.iter()
        .min_by_key(|ref a|
          strs.iter()
              .map(|ref b|
                hamming_distance(&a, &b).expect("Error calculating string distance"))
              .sum::<usize>())
        .and_then(|r|
          Some(r.to_string()))
}

#[cfg(not(test))]
fn main() {
    let input = stdin();
    let mut in_itr = input.lock().lines();
    if let Ok(c) = in_itr.next().expect("Err Reading on STDIN") {
        let count = c.parse::<usize>().expect("Missing Leading String Count");
        let v: Vec<String> = in_itr.take(count).map(|s| s.unwrap()).collect();
        println!("Center -> {}", find_center(&v).unwrap());
    }
}

#[cfg(test)]
mod test {

 use super::hamming_distance;
 use super::find_center;

 #[test]
 fn test_identity_distance() {
     let r = hamming_distance("ATCAATATCAA", "ATCAATATCAA");
     assert!(r.is_ok());
     assert!(0 == r.unwrap());
 }

 #[test]
 fn test_simple_distance() {
     let r = hamming_distance("ATCAATATCAA", "ATCAAUATCAA");
     assert!(r.is_ok());
     assert!(1 == r.unwrap());
 }

 #[test]
 fn test_find_center() {
     let v = vec!["ATCAATATCAA".to_string(),
                  "ATTAAATAACT".to_string(),
                  "AATCCTTAAAC".to_string(),
                  "CTACTTTCTTT".to_string(),
                  "TCCCATCCTTT".to_string(),
                  "ACTTCAATATA".to_string()];
     assert_eq!(find_center(&v).unwrap(), "ATTAAATAACT");
 }
}
