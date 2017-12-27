    extern crate rand;
    
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::File;
    use std::str::FromStr;
    use rand::distributions::{IndependentSample, Range};
    
    static INFILE : &str = "mozart-dice-starting.txt";
    static BEATS_PER_MEASURE : f32 = 3.0;
    static MEASURE_TABLE : [[u32;11];16] = [ [96, 32, 69, 40, 148, 104, 152, 119, 98, 3, 54],
                                             [22, 6, 95, 17 ,74 ,157 ,60 ,84 ,142 ,87 ,130],
                                             [141, 128 ,158 ,113 ,163 ,27 ,171 ,114 ,42 ,165 ,10],
                                             [41, 63 ,13 ,85 ,45 ,167 ,53 ,50 ,156 ,61 ,103],
                                             [105, 146 ,153 ,161 ,80 ,154 ,99 ,140 ,75 ,135 ,28],
                                             [122, 46 ,55 ,2 ,97 ,68 ,133 ,86 ,129 ,47 ,37],
                                             [11, 134 ,110 ,159 ,36 ,118 ,21 ,169 ,62 ,147 ,106],
                                             [30, 81 ,24 ,100 ,107 ,91 ,127 ,94 ,123 ,33 ,5],
                                             [70, 117 ,66 ,90 ,25 ,138 ,16 ,120 ,65 ,102 ,35],
                                             [121, 39 ,136 ,176 ,143 ,71 ,155 ,88 ,77 ,4 ,20],
                                             [26, 126 ,15 ,7 ,64 ,150 ,57 ,48 ,19 ,31 ,108],
                                             [9, 56 ,132 ,34 ,125 ,29 ,175 ,166 ,82 ,164 ,92],
                                             [112, 174 ,73 ,67 ,76 ,101 ,43 ,51 ,137 ,144 ,12],
                                             [49, 18 ,58 ,160 ,136 ,162 ,168 ,115 ,38 ,59 ,124],
                                             [109, 116 ,145 ,52 ,1 ,23 ,89 ,72 ,149 ,173 ,44],
                                             [14, 83 ,79 ,170 ,93 ,151 ,172 ,111 ,8 ,78 ,131], ];
    struct Note {
      pitch   : String,
      start   : f32,
      duration: f32, 
    }
    
    impl Note {
      fn measure(&self) -> u32 {
        (self.start / BEATS_PER_MEASURE).floor() as u32
      }
    
      fn to_measure(&self, m : u32) -> Note {
        let beat = self.start - (self.measure() as f32 * BEATS_PER_MEASURE);
        let s = (m as f32 * BEATS_PER_MEASURE) + beat;
        Note { pitch: self.pitch.to_string(), start: s, duration : self.duration }
      }
    
    }
    
    
    impl ToString for Note {
      fn to_string( &self ) -> String {
        format!( "{} {:.1} {:.1}", self.pitch, self.start, self.duration ).to_string()
      }
    }
    
    struct ParseNoteError;
    
    impl FromStr for Note {
    
      type Err = ParseNoteError;
    
      fn from_str( s: &str ) -> Result<Self, Self::Err> {
        let mut words =  s.split_whitespace();
        if let Some( pitch ) = words.next().and_then(|p| Some( p.to_string() )) {
          if let Some( start ) = words.next().and_then( |s| s.parse::<f32>().ok() ) {
            if let Some( duration ) = words.next().and_then( |d| d.parse::<f32>().ok() ) {
              return Ok( Note { pitch: pitch, start: start, duration: duration } )
    	}
          }
        }
        Err( ParseNoteError )
      }
    }
    
    fn read_music_file( filename : &str ) -> Result< Vec< Note >, String > {
      // file io stuff
      let f = File::open( filename ).expect( "Unable to open input file" );
      let mut input = BufReader::new( f );
      // music score
      let mut score = Vec::new();
      loop {
        let mut buf = String::new();
        let bytes_read = input.read_line( &mut buf ).expect("Error reading file");
        if bytes_read == 0 { return Ok( score ) }     
        if let Ok( note ) = buf.parse::< Note >() {
          score.push( note );
        }
      }
    }
    
    fn main() {
      let mut rng = rand::thread_rng();
      let d6 = Range::new(1,6);
    
      if let Ok( score ) = read_music_file( INFILE ) {
        for row in 0..MEASURE_TABLE.len() {
          let two_d6 : usize = d6.ind_sample(&mut rng) + d6.ind_sample(&mut rng);
          for note in score.iter().filter( |n| n.measure() == MEASURE_TABLE[row][two_d6] ) {
            println!("{}", note.to_measure(row as u32).to_string());
          }
        }
      } else {
        println!( "Err reading file" );
      }
    }
