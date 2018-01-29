    enum Bounds {
      Less,
      LessOrEqual,
      Equal,
      GreaterOrEqual,
      Greater,
    }
    
    
    // Greedy recursive solution
    //  Not optimal, but fast.
    fn greedy_r(bag : &Vec<i32>, options : &Vec<i32>, target :i32) -> Option<Vec<i32>> {
      if options.len() <= 0 {
        return None
      }
      let mut b = bag.clone();
      let mut o = options.clone();
      let i = o.remove(0);
      let count = b.iter().sum::<i32>() + i;
      if count < target {
        b.push(i);
        return greedy_r(&b, &o, target)
      }
      if count > target {
        return greedy_r(&b, &o, target)
      }
      // count == target!
      b.push(i);
      Some(b)
    }
    
    // generate all powersets
    // code from user 'erip' on StackOverflow
    fn powerset<T>(s: &[T]) -> Vec<Vec<T>> where T: Clone {
        (0..2usize.pow(s.len() as u32)).map(|i| {
             s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
                                 .map(|(_, element)| element.clone())
                                 .collect()
         }).collect()
    }  
    
    fn make_change(amt : i32, coins : &Vec<i32>, count : usize, bounds : Bounds ) -> Vec<i32> {
      // first try a greedy solution, failing that go for exhaustive
      let mut cs = coins.clone();
      cs.sort();
      cs.reverse();
      let sets = greedy_r(&vec![], &cs, amt).and_then(|v| Some(vec![v])).unwrap_or(powerset(&coins)); 
      // prune down possible solution space
      let mut sols = sets.iter().filter(|s| 
        match bounds {
          Bounds::Less           => return s.len() <  count,
          Bounds::LessOrEqual    => return s.len() <= count,
          Bounds::Equal          => return s.len() == count,
          Bounds::GreaterOrEqual => return s.len() >= count,
          Bounds::Greater        => return s.len() >  count,
        }).filter(|s| s.iter().sum::<i32>() == amt);
      sols.next().expect("No solution possible!").clone()
    }
    
    #[cfg(not(test))]
    fn main() {
       println!("Make Change!"); 
       let c = make_change(150, &vec![1; 10000], 150, Bounds::Equal);
       println!("->{:?}", c);
    }

    #[cfg(test)]
    mod test {
    
      use super::{Bounds,make_change};
    
      #[test]
      fn test_one() {
        let r = make_change(150, &vec![100, 50, 50, 50, 50], 5, Bounds::Less);
        assert!(r.len() < 5);
        assert!(r.iter().sum::<i32>() == 150);
      }
    
      #[test]
      fn test_two() {
        let r = make_change(130, &vec![100, 20, 18, 12, 5, 5], 6, Bounds::Less);
        assert!(r.len() < 6);
        assert!(r.iter().sum::<i32>() == 130);
      }
    
      #[test]
      #[should_panic]
      fn test_three() {
        let r = make_change(200, &vec![50, 50, 20, 20, 10], 5, Bounds::GreaterOrEqual);
      }

      #[test]
      fn test_force_powerset() {
        let r = make_change(110, &vec![90, 18, 12, 8, 5], 3, Bounds::Equal);
        assert!(r.len() == 3);
        assert!(r.iter().sum::<i32>() == 110);
      }
    
    }
