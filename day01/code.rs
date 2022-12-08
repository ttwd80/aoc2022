use std::cmp;
use std::io::stdin;

fn main() {
    let mut vec = read_vec();
    let n = vec.len();

    // ensure 3 largest ones are at the last 3 positions
    // 2 possible configuration
    // .. .. (3rd largest), (2nd largest), (largest)
    // or
    // .. .. (3rd largest), (largest), (2nd largest)
    // but either configuration will get us what we need
    // the sum of top 3
    vec.select_nth_unstable(n - 3);

    // largest is either the last element, or 2nd last element
    println!("part 1: {}", cmp::max(vec[n - 1], vec[n - 2]));

    println!("part 2: {}", (1..=3).map(|x| vec[n - x]).sum::<i32>());
}

fn read_vec() -> Vec<i32> {
    let mut block: i32 = 0;
    let mut vec: Vec<i32> = Vec::new();

    for line in stdin().lines() {
        let s = line.unwrap();
        if s.len() == 0 {
            vec.push(block);
            block = 0;
        } else {
            let value: i32 = s.parse().unwrap();
            block += value;
        }
    }
    if block > 0 {
        vec.push(block);
    }
    return vec;
}
