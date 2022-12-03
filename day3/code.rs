use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut result1 = 0 as i32;
    let mut result2 = 0 as i32;
    let target2 = calculate_target(2);
    let target3 = calculate_target(3);
    println!("{}", target2);
    println!("{}", target3);
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(s) = line {
                let ba = s.as_bytes();
                result1 = result1 + process(ba, 2, target2);
                result2 = result2 + process(ba, 3, target3);
            }
        }
    }
    println!("{}", result1);
    println!("{}", result2);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process(s: &[u8], parts: i32, target: i32) -> i32 {
    println!("{}", target);
    return s.len() as i32 + parts;
}

fn calculate_target(parts: i32) -> i32 {
    let mut total:i32 = 0;
    for n in 0..parts {
        total = total + n + 1;
    }
    return total;
}
