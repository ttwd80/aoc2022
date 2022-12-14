use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines_as_vector() -> Vec<String> {
    let mut vec = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line {
                vec.push(value);
            }
        }
    }
    vec
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let lines = read_lines_as_vector();
    let mut cwd: Vec<String> = Vec::new();
    let mut map_size: HashMap<String, i64> = HashMap::new();
    for line in lines {
        let mut parts = line.split(" ");
        if line.starts_with("$") {
            if line == "$ cd /" {
                cwd.clear();
            } else if line == "$ cd .." {
                cwd.pop();
            } else if line.starts_with("$ cd ") {
                let part = parts.nth(2).unwrap();
                cwd.push(String::from(part));
            }
            //command
        } else {
            //data
            if line.starts_with("dir") {
                //dir
            } else {
                let size: i64 = parts.nth(0).unwrap().parse::<i64>().unwrap();
                let n = cwd.len();
                for p in 0..n {
                    let key = &cwd[0..=p].to_vec().join("/");
                    map_size
                        .entry(String::from(key))
                        .and_modify(|v| *v += size)
                        .or_insert(size);
                }
                map_size
                    .entry(String::from("/"))
                    .and_modify(|v| *v += size)
                    .or_insert(size);
            }
        }
    }
    let total_space = 70000000;
    let total_used = map_size["/"];
    let total_free = total_space - total_used;
    let unused_target:i64 = 30000000;
    let mut sum1: i64 = 0;
    let mut sum2:i64 = total_space;
    for (key, value) in map_size {
        println!("{} - {}", key, value);
        if value <= 100000 {
            sum1 += value;
        }
        if total_free + value >= unused_target && value < sum2{
            sum2 = value;
        }
    }
    println!("result 1 => {}", sum1);
    println!("result 2 => {}", sum2);
}
