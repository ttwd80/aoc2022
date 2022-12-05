use std::collections::VecDeque;
use std::io::stdin;

fn main() {
    let lines = read_lines();

    //given the lines,
    //i want several things
    //- a list of string for instructions
    //- an array of VecDeque to represent the stacks
    let (commands, stacks) = parse_lines(lines.clone());
    let result1 = process(commands.clone(), stacks.clone(), false);
    println!("result1 -> {}", result1);

    let (commands, stacks) = parse_lines(lines.clone());
    let result2 = process(commands.clone(), stacks.clone(), true);
    println!("result2 -> {}", result2);
}

fn push_front(vec: &mut VecDeque<char>, c: char) {
    vec.push_front(c);
}

fn push_back(vec: &mut VecDeque<char>, c: char) {
    vec.push_back(c);
}

fn configure_push(fifo: bool) -> fn(&mut VecDeque<char>, char) {
    if fifo {
        return push_front;
    } else {
        return push_back;
    }
}

fn process(commands: Vec<String>, mut stacks: Vec<VecDeque<char>>, fifo: bool) -> String {
    let n = commands.len();

    for i in 0..n {
        let split: Vec<&str> = commands[i].split(" ").collect();
        let count = split[1].parse::<i32>().unwrap();
        let source = split[3].parse::<i32>().unwrap() as usize;
        let target = split[5].parse::<i32>().unwrap() as usize;

        let mut vec_deque: VecDeque<char> = VecDeque::new();
        let push_to_temp = configure_push(fifo);

        for _ in 0..count {
            let v = stacks[source - 1].pop_front().unwrap();
            push_to_temp(&mut vec_deque, v);
        }
        for _ in 0..count {
            let v = vec_deque.pop_front().unwrap();
            stacks[target - 1].push_front(v);
        }
    }
    let mut result = String::from("");
    let total_stack_count = stacks.len();
    for i in 0..total_stack_count {
        let c: char = *stacks[i].front().unwrap();
        result.push(c);
    }
    return result;
}

fn read_lines() -> Vec<String> {
    let mut lines = Vec::new();
    for line in stdin().lines() {
        lines.push(line.unwrap());
    }
    return lines;
}

fn parse_lines(lines: Vec<String>) -> (Vec<String>, Vec<VecDeque<char>>) {
    let mut commands = Vec::new();
    let mut stacks = Vec::new();
    for _ in 0..9 {
        let deque: VecDeque<char> = VecDeque::new();
        stacks.push(deque);
    }
    for row in 0..8 {
        for col in 0..9 {
            let b = lines[row].as_bytes();
            let c = b[(col * 4) + 1] as char;
            if c != ' ' {
                stacks[col].push_back(c);
            }
        }
    }
    let n = lines.len();
    for i in 10..n {
        commands.push(lines[i].clone());
    }
    return (commands, stacks);
}
