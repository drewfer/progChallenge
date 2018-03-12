
/**
 * Fermat's Factorization Method
 * N must be odd!
 *
 * info - https://en.wikipedia.org/wiki/Fermat%27s_factorization_method
 */
fn fermat_factor(i: i32) -> (i32, i32) {
    let n = i as f64;
    let mut a = n.sqrt().ceil();
    let mut b = a * a - n;
    while b != b.sqrt().floor().powf(2.0) {
        a += 1.0;
        b = a * a - n;
    }
    let x  = a - b.sqrt();
    let y  = a + b.sqrt();

    if x * y != n {
        (1, i)
    } else {
        (x as i32, y as i32)
    }
}

fn trial_factor(n: i32) -> Vec<(i32,i32)> {
    let root = (n as f32).sqrt().ceil() as i32;

    (0..root).enumerate()
        .skip(1)
        .filter(|&(_, j)| {
            n.wrapping_rem(j) == 0
        }).map(|(i, j)| {
            (i as i32, n.wrapping_div(j) )
        }).collect()
}


/**
 *  Given a number A, find the smallest possible value of B+C, if B*C = A.
 */
fn min_factors(a: i32) -> (i32,i32) {
    if a.wrapping_rem(2) == 1 { // is_odd
        return fermat_factor(a)
    }
    *trial_factor(a).iter().min_by_key(|i|{i.0 + i.1}).unwrap()
}

fn main() {
    let n = vec![12, 456, 4567, 12345];
    for i in n {
        let (a, b) = min_factors(i);
        println!("{} => {}", i, a+b);
    }
}

#[cfg(test)]
mod test {

 use super::fermat_factor;
 use super::trial_factor;

 #[test]
 fn test_fermat_factor() {
     let (a, b) = fermat_factor(5959);
     assert_eq!(59, a);
     assert_eq!(101, b);

 }

 #[test]
 fn test_fermat_factor_large_prime() {
     let largest_prime : i32 = 2147483647;
     let (a, b) = fermat_factor(largest_prime);
     assert_eq!(1, a);
     assert_eq!(largest_prime, b);

 }

 #[test]
 fn test_trial_factor() {
     let v = trial_factor(5959); //v == vec![(1, 5959),(50,101)]
     assert_eq!(1, v[0].0);
     assert_eq!(5959, v[0].1);
     assert_eq!(59, v[1].0);
     assert_eq!(101, v[1].1);

 }



}
