use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut result1:i32 = 0;
    let mut result3:i32 = 0;
    let target1 = calculate_target(2);
    let target2 = calculate_target(3);
    println!("{}", target1);
    let mut block3: Vec<String> = Vec::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(s) = line {
                let ba = s.as_bytes();
                // println!("{}", s);
                block3.push(s);
                if block3.len() == 3 {
                    result3 += process3(&block3, target2);
                    block3.truncate(0);
                    // break;
                }

            }
        }
    }
    // result3 += process3(&block3, target2);
    println!("{}", result1);
    println!("{}", result3);
}

fn process3(v: &Vec<String>, target: i32) -> i32 {
    let mut block:[u8;128] = [0; 128];
    let v0 = &v[0];
    let n0 = v0.len();
    for i in 0..n0 {
        let c = v0.chars().nth(i).unwrap() as usize;
        block[c] |= 1;
    }
    let v1 = &v[1];
    let n1  = v1.len();
    for i in 0..n1 {
        let c = v1.chars().nth(i).unwrap() as usize;
        block[c] |= 2;
    }
    let v2 = &v[2];
    let n2 = v2.len();
    for i in 0..n2 {
        let c = v2.chars().nth(i).unwrap() as usize;
        block[c] |= 4;
        if block[c] == 7 {
            println!("{}", (c as u8) as char);
            return change(c as i32);
        }
    }
    return 100;
}

fn change(x: i32) -> i32 {
    if x <= 90 {
        return x - 64 + 26
    } else {
        return x - 96;
    }
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process(s: &[u8], parts: i32, target: i32) -> i32 {
    let mut array: [u8; 128] = [0; 128];
    let mut marker = vec![0 as u8; parts as usize];
    let mut m = 1;
    for n in 0..parts {
        marker[n as usize] = m;
        m = m + m;
    }
    let count = (s.len() / parts as usize) as i32;
    for e in 0..count {
        for p in 0..parts {
            let offset = ((p * parts) + count) as usize;
            let value = s[offset];
            array[value as usize] = array[value as usize] | marker[p as usize];
            if marker[p as usize] == target as u8 {
                return value.into();
            }
        }
    }
    println!("error");
    return s.len() as i32 + parts;
}

fn calculate_target(parts: i32) -> i32 {
    let mut total:i32 = 0;
    let mut b = 1;
    for n in 0..parts {
        total = total + b;
        b = b + b;
    }
    return total;
}
