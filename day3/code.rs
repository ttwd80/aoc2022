use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let mut total = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(s) = line {
                // println!("{}", ip);
                let mut ha = HashSet::new();
                let mut hz = HashSet::new();
                let n = s.len();
                let b = s.as_bytes();
                let mut a = 0 as usize;
                let mut z = n - 1 as usize;
                let mut value = 0 as u8;
                while a < z {
                    let ba = b[a];
                    let bz = b[z];
                    if hz.contains_key(ba) {
                        value = ba;
                        break;
                    } else {
                        ha.insert(ba);
                    }
                    if ha.contains_key(bz) {
                        value = bz;
                        break;
                    } else {
                        hz.insert(bz);
                    }
                    a = a + 1;
                    z = z - 1;
        
                }
                let mut plus = 0 as u32;
                println!("{}", value);
                if value <= 90 {
                    plus = value - 64 + 26;
                } else {
                    plus = value - 96;
                }
                total = total + plus;    
                println!("{} - {} - {}", (value as u8) as char, plus, total);
            }
        }
    }
    println!("{}", total);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
