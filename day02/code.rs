use std::io::stdin;

fn main() {
    let lines = read_lines();
    let (result1, result2) = process(lines);
    println!("result 1 {}", result1);
    println!("result 2 {}", result2);
}

fn win_status(op: i32, me: i32) -> i32 {
    if op == me {
        return 3;
    } else {
        if me - op == 1 || me - op == -2 {
            return 6;
        } else {
            return 0;
        }
    }
}

fn pick_to_get_status(op: i32, status: i32) -> i32 {
    if status == 1 {
        return op;
    } else if status == 2 {
        if op == 2 {
            return 0;
        } else {
            return op + 1;
        }
    } else {
        if op == 0 {
            return 2;
        } else {
            return op - 1;
        }
    }
}

fn process(lines: Vec<String>) -> (i32, i32) {
    let n = lines.len();
    let mut count1: i32 = 0;
    let mut count2: i32 = 0;
    for i in 0..n {
        let line = &lines[i];
        // println!("{} --", line);
        let b = line.as_bytes();
        let op = b[0] - 65;
        let me = b[2] - 88;
        // println!("{} {}", op, me);
        count1 += win_status(op.into(), me.into());
        count1 += me as i32 + 1;

        //0 1 2
        //lose, draw, win
        let my_pick = pick_to_get_status(op.into(), me.into());
        count2 += me as i32 * 3;
        count2 += my_pick + 1;
        println!("op = {}, desired = {}, so pick = {}", op, me as i32 * 3, my_pick);
    }
    return (count1, count2);
}

fn read_lines() -> Vec<String> {
    let mut lines = Vec::new();
    for line in stdin().lines() {
        lines.push(line.unwrap())
    }
    return lines;
}
