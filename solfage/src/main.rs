    // Chromatic notes
    static NOTES : [&str;12]  = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];  
    
    // [(solfage, semitones)]
    static SOLFAGE : [(&str, usize);7] = [("Do", 0), ("Re", 2), ("Mi", 4), ("Fa", 5), ("So", 7), ("La", 9), ("Ti", 11)]; 
    
    //// given a scale, returns the note of the given degree 
    fn note(scale : &str, degree : &str) -> String
    {
       let tonic = NOTES.iter().position(|&n| n == scale).expect("Invalid Scale");
       let semitones = SOLFAGE.iter().find(|&t| t.0 == degree).expect("Invalid Degree").1;
       
       NOTES[(tonic + semitones) % NOTES.len()].to_string()
    }
    
    
    #[cfg(not(test))]
    fn main() {
        println!("{}", note("D", "Do"));
        println!("{}", note("D", "Re"));
        println!("{}", note("D", "Mi"));
        println!("{}", note("D", "Fa"));
        println!("{}", note("D", "So"));
        println!("{}", note("D", "La"));
        println!("{}", note("D", "Ti"));
    }
    
    
    #[cfg(test)]
    mod test {
    
     use super::note;
    
     #[test]
     fn test_c_scale() {
         assert!("C" == note("C", "Do"));
         assert!("D" == note("C", "Re"));
         assert!("E" == note("C", "Mi"));
         assert!("F" == note("C", "Fa"));
         assert!("G" == note("C", "So"));
         assert!("A" == note("C", "La"));
         assert!("B" == note("C", "Ti"));
     }
    
     #[test]
     fn test_incidental() {
         assert!("F#" == note("D", "Mi"));
         assert!("D#" == note("A#", "Fa"));
     }
    
     #[test]
     fn test_wrap() {
         assert!("A" == note("G", "Re"));
     }
    
     #[test]
     #[should_panic]
     fn test_bad_note(){
         assert!("F#" == note("B#", "Mi"));
     }
    
     #[test]
     #[should_panic]
     fn test_bad_degree(){
         assert!("E" == note("C", "Ni"));
     }
    
    }
