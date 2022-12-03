use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut total = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(s) = line {
                // println!("{}", ip);
                let x:[u8;128] = [0;128];
                let n = s.len();
                let b = s.as_bytes();
                let mut a = 0;
                let mut z = n - 1;
                let mut value = 0;
                while a < z {
                    let pa = b[a] as i8;
                    if x[pa] == 1 {
                        value = pa;
                        break;
                    } else {
                        x[pa] = 1;
                        a = a + 1;
                    }
                    
                    let pz = b[z] as i8;
                    if x[pb] == 1 {
                        value = pb;
                    } else {
                        x[pb] = 1;
                        z = z - 1;
                    }                        
                }
                if value <= 122 {
                    total = total + value - 96;
                } else {
                    total = total + value - 64;
                }
                    
                // println!("{}", n);
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
