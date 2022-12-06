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

    //process every single character in the line, once
    for i in 0..n {

        //try to put item in the map
        let c = b[i] as char;

        // if it did not exist, put it as 1
        // if it already existed, increment it
        if !map.contains_key(&c) {
            map.insert(c, 1);
        } else {
            let value = *map.get(&c).unwrap();
            map.insert(c, value + 1);
        }

        // we have enough range from start to end to maybe start removing
        if i - a >= count {

            //reduce
            let ca = b[a] as char;
            let value = *map.get(&ca).unwrap();
            // if it is the last one, remove it
            if value == 1 {
                map.remove(&ca);
            } else {
                // if not the last one, decrement it
                map.insert(ca, value - 1);
            }
            // start should be incremented to the next character
            a += 1;
        }
        // if we have distinct items, we hit our mark
        if map.len() == count {
            return (i + 1) as i32;
        }
    }
    return 0;
}
