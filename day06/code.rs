use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let lines = read_lines();
    let result1 = process(&lines, 4);
    let result2 = process(&lines, 14);
    println!("result 1 {}", result1);
    println!("result 2 {}", result2);
}
fn read_lines() -> Vec<String> {
    let mut lines = Vec::new();
    for line in stdin().lines() {
        lines.push(line.unwrap());
    }
    return lines;
}

fn process(vec: &Vec<String>, count: usize) -> i32 {
    let line = &vec[0];
    let n = line.len();

    let mut map = HashMap::new();

    let b = line.as_bytes();
    let mut a = 0;
    for i in 0..n {
        //try to put item in the map
        let c = b[i] as char;

        if !map.contains_key(&c) {
            map.insert(c, 1);
        } else {
            let value = *map.get(&c).unwrap();
            map.insert(c, value + 1);
        }
        if i - a >= count {
            //reduce
            let ca = b[a] as char;
            let value = *map.get(&ca).unwrap();
            if value == 1 {
                map.remove(&ca);
            } else {
                map.insert(ca, value - 1);
            }
            a += 1;
        }
        if map.len() == count {
            return (i + 1) as i32;
        }
    }

    return 0;
}
