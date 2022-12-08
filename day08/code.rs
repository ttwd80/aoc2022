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

    let rows = lines.len() as i32;
    let cols = lines[0].len() as i32;

    let mut values = vec![vec![0 as u8; cols as usize]; rows as usize];

    for r in 0..rows as usize {
        let b = lines[r].as_bytes();
        for c in 0..cols as usize {
            let v = b[c] as u8 - '0' as u8;
            values[r][c] = v;
        }
    }

    let mut result1 = 0;
    let mut result2 = 0;
    for r in 0..rows as i32 {
        for c in 0..cols as i32 {
            result1 += process_1(&values, r, c, rows, cols);
            let result = process_2(&values, r, c, rows, cols);
            if result2 < result {
                result2 = result;
            }
        }
    }
    println!("Result 1 -> {}", result1);
    println!("Result 2 -> {}", result2);
}

fn process_1(values: &Vec<Vec<u8>>, r: i32, c: i32, rows: i32, cols: i32) -> i32 {
    if process_1_count(values, r, c, rows, cols, 0, 1)
        || process_1_count(values, r, c, rows, cols, 0, -1)
        || process_1_count(values, r, c, rows, cols, 1, 0)
        || process_1_count(values, r, c, rows, cols, -1, 0)
    {
        return 1;
    }
    return 0;
}
fn process_1_count(
    values: &Vec<Vec<u8>>,
    r: i32,
    c: i32,
    rows: i32,
    cols: i32,
    rc: i32,
    cc: i32,
) -> bool {
    let mut nc = c + cc;
    let mut nr = r + rc;
    let value = values[r as usize][c as usize];
    while nc >= 0 && nc < cols && nr >= 0 && nr < rows {
        let ri = nr as usize;
        let ci = nc as usize;
        if values[ri][ci] >= value {
            return false;
        }
        nc = nc + cc;
        nr = nr + rc;
    }
    return true;
}

fn process_2(values: &Vec<Vec<u8>>, r: i32, c: i32, rows: i32, cols: i32) -> i32 {
    let d1 = process_2_count(values, r, c, rows, cols, -1, 0); //U
    let d2 = process_2_count(values, r, c, rows, cols, 0, -1); //L
    let d3 = process_2_count(values, r, c, rows, cols, 0, 1); //R
    let d4 = process_2_count(values, r, c, rows, cols, 1, 0); //D
    let p = d1 * d2 * d3 * d4;
    return p;
}

fn process_2_count(
    values: &Vec<Vec<u8>>,
    r: i32,
    c: i32,
    rows: i32,
    cols: i32,
    rc: i32,
    cc: i32,
) -> i32 {
    let mut count = 0;
    let mut nc = c + cc;
    let mut nr = r + rc;
    let value = values[r as usize][c as usize];
    while nc >= 0 && nc < cols && nr >= 0 && nr < rows {
        count += 1;
        let ri = nr as usize;
        let ci = nc as usize;
        if values[ri][ci] >= value {
            return count;
        }
        nc = nc + cc;
        nr = nr + rc;
    }
    return count;
}
