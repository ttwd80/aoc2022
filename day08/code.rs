use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// cat expected-0.txt| sed 's/],/],\n/g' > expected-0-f.txt
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

    let rows = lines.len() as usize;
    let cols = lines[0].len() as usize;

    let mut trees = vec![vec![0 as u8; cols]; rows];

    for r in 0..rows {
        let line = &lines[r];
        let b = line.as_bytes();
        for c in 0..cols {
            trees[r][c] = b[c] - '0' as u8;
        }
    }

    struct DirColInc {
        rows: usize,
        cols: usize,
        data: Vec<Vec<i32>>,
    }
    struct DirColDec {
        rows: usize,
        cols: usize,
        data: Vec<Vec<i32>>,
    }
    struct DirRowInc {
        rows: usize,
        cols: usize,
        data: Vec<Vec<i32>>,
    }
    struct DirRowDec {
        rows: usize,
        cols: usize,
        data: Vec<Vec<i32>>,
    }

    trait Direction {
        fn to_string(&self) -> String;

        // we have 4 structs that implement this trait
        // DirColInc/DirColDec/DirRowInc/DirRowDec
        // 2 increment/decrement cols
        // 2 increment/decrement rows
        // this returns the rows for DirCol*
        // this returns the cols for DirRow*
        fn range_change_slow(&self) -> usize;

        // we have 4 structs that implement this trait
        // DirColInc/DirColDec/DirRowInc/DirRowDec
        // 2 increment/decrement cols
        // 2 increment/decrement rows
        // this returns the rows for DirRow*
        // this returns the cols for DirCol*
        fn range_change_fast(&self) -> usize;

        //this maps to (row, col)
        //for DirColInc -> (slow, fast) -> (slow, fast)
        //for DirColDec -> (slow, fast) -> (slow, cols - 1 - fast), fast from high to low
        //for DirRowInc -> (slow, fast) -> (fast, slow)
        //for DirRowDec -> (slow, fast) -> (rows - fast - 1, slow)
        fn to_row_col(&self, slow: usize, fast: usize) -> (usize, usize);

        //For DirColInc, DirColDec, returns a value of a cell in the current row
        //For DirRowInc, DirRowDec, returns a value of a cell in the current col
        //This value will determine if the stack should be popped
        fn top(&self, trees: &Vec<Vec<u8>>, stack: &Vec<usize>, row: usize, col: usize) -> u8;

        //If the stack is empty, set the cell at this location to a specific value
        //for DirColInc -> c
        //for DirColDec -> cols - c - 1
        //for DirRowInc -> r
        //for DirRowDec -> rows - r - 1
        fn set(&mut self, row: usize, col: usize);

        //Figure out the top value
        //set the cell value based on the row or column value and the top value
        //DirColInc - top is smaller and col larger: col - top.
        //DirColDec - top is larger and col smaller: top - col.
        //DirRowInc - top is smaller and row is larger: row - top
        //DirRowDec - top is larger and row is smaller: top - row
        fn set_with_top(&mut self, row: usize, col: usize, top: i32);

        //Push what we just processed
        //DirColInc - col
        //DirColDec - col
        //DirRowInc - row
        //DirRowDec - row
        fn push(&self, stack: &mut Vec<usize>, row: usize, col: usize);

        fn get(&self, row: usize, col: usize) -> i32;
    }

    impl Direction for DirColInc {
        fn to_string(&self) -> String {
            format!("{:?}", self.data)
        }
        fn range_change_fast(&self) -> usize {
            self.cols
        }
        fn range_change_slow(&self) -> usize {
            self.rows
        }
        fn to_row_col(&self, slow: usize, fast: usize) -> (usize, usize) {
            (slow, fast)
        }
        fn top(&self, trees: &Vec<Vec<u8>>, stack: &Vec<usize>, row: usize, _col: usize) -> u8 {
            trees[row][*stack.last().unwrap()]
        }
        fn set(&mut self, row: usize, col: usize) {
            self.data[row][col] = col as i32;
        }
        fn set_with_top(&mut self, row: usize, col: usize, top: i32) {
            self.data[row][col] = col as i32 - top;
        }
        fn push(&self, stack: &mut Vec<usize>, _row: usize, col: usize) {
            stack.push(col);
        }
        fn get(&self, row: usize, col: usize) -> i32 {
            self.data[row][col]
        }
    }

    impl Direction for DirColDec {
        fn to_string(&self) -> String {
            format!("{:?}", self.data)
        }
        fn range_change_fast(&self) -> usize {
            self.cols
        }
        fn range_change_slow(&self) -> usize {
            self.rows
        }
        fn to_row_col(&self, slow: usize, fast: usize) -> (usize, usize) {
            (slow, self.cols - fast - 1)
        }
        fn top(&self, trees: &Vec<Vec<u8>>, stack: &Vec<usize>, row: usize, _col: usize) -> u8 {
            trees[row][*stack.last().unwrap()]
        }
        fn set(&mut self, row: usize, col: usize) {
            self.data[row][col] = self.cols as i32 - col as i32 - 1;
        }
        fn set_with_top(&mut self, row: usize, col: usize, top: i32) {
            self.data[row][col] = top - col as i32;
        }
        fn push(&self, stack: &mut Vec<usize>, _row: usize, col: usize) {
            stack.push(col);
        }
        fn get(&self, row: usize, col: usize) -> i32 {
            self.data[row][col]
        }
    }

    impl Direction for DirRowInc {
        fn to_string(&self) -> String {
            format!("{:?}", self.data)
        }
        fn range_change_fast(&self) -> usize {
            self.rows
        }
        fn range_change_slow(&self) -> usize {
            self.cols
        }
        fn to_row_col(&self, slow: usize, fast: usize) -> (usize, usize) {
            (fast, slow)
        }
        fn top(&self, trees: &Vec<Vec<u8>>, stack: &Vec<usize>, _row: usize, col: usize) -> u8 {
            trees[*stack.last().unwrap()][col]
        }
        fn set(&mut self, row: usize, col: usize) {
            self.data[row][col] = row as i32;
        }
        fn set_with_top(&mut self, row: usize, col: usize, top: i32) {
            self.data[row][col] = row as i32 - top;
        }
        fn push(&self, stack: &mut Vec<usize>, row: usize, _col: usize) {
            stack.push(row);
        }
        fn get(&self, row: usize, col: usize) -> i32 {
            self.data[row][col]
        }
    }

    impl Direction for DirRowDec {
        fn to_string(&self) -> String {
            format!("{:?}", self.data)
        }
        fn range_change_fast(&self) -> usize {
            self.rows
        }
        fn range_change_slow(&self) -> usize {
            self.cols
        }
        fn to_row_col(&self, slow: usize, fast: usize) -> (usize, usize) {
            (self.rows - 1 - fast, slow)
        }
        fn top(&self, trees: &Vec<Vec<u8>>, stack: &Vec<usize>, _row: usize, col: usize) -> u8 {
            trees[*stack.last().unwrap()][col]
        }
        fn set(&mut self, row: usize, col: usize) {
            self.data[row][col] = self.rows as i32 - row as i32 - 1;
        }
        fn set_with_top(&mut self, row: usize, col: usize, top: i32) {
            self.data[row][col] = top - row as i32;
        }
        fn push(&self, stack: &mut Vec<usize>, row: usize, _col: usize) {
            stack.push(row);
        }
        fn get(&self, row: usize, col: usize) -> i32 {
            self.data[row][col]
        }
    }

    let dir0: DirColInc = DirColInc {
        rows,
        cols,
        data: vec![vec![0 as i32; cols]; rows],
    };
    let dir1: DirColDec = DirColDec {
        rows,
        cols,
        data: vec![vec![0 as i32; cols]; rows],
    };
    let dir2: DirRowInc = DirRowInc {
        rows,
        cols,
        data: vec![vec![0 as i32; cols]; rows],
    };
    let dir3: DirRowDec = DirRowDec {
        rows,
        cols,
        data: vec![vec![0 as i32; cols]; rows],
    };

    let mut dirs: Vec<Box<dyn Direction>> = Vec::new();
    dirs.push(Box::new(dir0));
    dirs.push(Box::new(dir1));
    dirs.push(Box::new(dir2));
    dirs.push(Box::new(dir3));
    for i in 0..dirs.len() {
        let d = &mut dirs[i];
        for slow in 0..d.range_change_slow() {
            let mut stack: Vec<usize> = Vec::new();
            for fast in 0..d.range_change_fast() {
                let (row, col) = d.to_row_col(slow, fast);

                while !stack.is_empty() && d.top(&trees, &stack, row, col) < trees[row][col] {
                    stack.pop();
                }
                if stack.is_empty() {
                    d.set(row, col);
                } else {
                    let top = *stack.last().unwrap() as i32;
                    d.set_with_top(row, col, top);
                }
                d.push(&mut stack, row, col);
            }
        }
    }
    let mut result1 = 0;
    let mut result2 = 0;
    for r in 0..rows {
        for c in 0..cols {
            let v0 = dirs[0].get(r, c);
            let v1 = dirs[1].get(r, c);
            let v2 = dirs[2].get(r, c);
            let v3 = dirs[3].get(r, c);
            let product = v0 * v1 * v2 * v3;
            if (v0 == c as i32 && (trees[r][c] > trees[r][0]))
                || (v1 == cols as i32 - c as i32 - 1 && trees[r][c] > trees[r][cols - 1])
                || (v2 == r as i32 && trees[r][c] > trees[0][c])
                || (v3 == rows as i32 - r as i32 - 1 && trees[r][c] > trees[rows - 1][c])
                || r == 0
                || c == 0
                || r == rows - 1
                || c == cols - 1
            {
                result1 += 1;
            }
            result2 = cmp::max(result2, product);
        }
    }
    println!("result 1 => {}", result1);
    println!("result 2 => {}", result2);
}
