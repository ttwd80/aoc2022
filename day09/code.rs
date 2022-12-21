use std::{
    collections::{HashMap, HashSet},
    fs,
};
struct Entry {
    direction: String,
    value: u8,
}

fn to_entry(line: &str) -> Entry {
    let parts: Vec<&str> = line.split(" ").collect();
    return Entry {
        direction: String::from(parts[0]),
        value: parts[1].parse().unwrap(),
    };
}
fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<&str> = content.split("\n").collect();
    let items: Vec<Entry> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| to_entry(line))
        .collect();

    let mut moves1: HashSet<String> = HashSet::new();
    let mut moves9: HashSet<String> = HashSet::new();
    let dir_u = [0 + 0, 0 + 1];
    let dir_d = [0 + 0, 0 - 1];
    let dir_l = [0 - 1, 0 + 0];
    let dir_r = [0 + 1, 0 + 0];
    let dir_diag1 = [0 - 1, 0 - 1];
    let dir_diag2 = [0 - 1, 0 + 1];
    let dir_diag3 = [0 + 1, 0 - 1];
    let dir_diag4 = [0 + 1, 0 + 1];
    let direction_map = HashMap::from([("U", dir_u), ("D", dir_d), ("L", dir_l), ("R", dir_r)]);
    let move_map = HashMap::from([
        (true, [dir_u, dir_d, dir_l, dir_r]),
        (false, [dir_diag1, dir_diag2, dir_diag3, dir_diag4]),
    ]);

    let mut rope = [[0; 2]; 10];

    moves1.insert(format!("(0,0)"));
    moves9.insert(format!("(0,0)"));

    for x in items {
        let d = direction_map.get(x.direction.as_str()).unwrap();
        for _m in 0..x.value {
            rope[0][0] += d[0];
            rope[0][1] += d[1];

            for r in 1..rope.len() {
                let hx = rope[r - 1][0];
                let hy = rope[r - 1][1];
                let tx = rope[r][0];
                let ty = rope[r][1];
                if !is_touching(hx, hy, tx, ty) {
                    let move_type_simple;
                    if hx == tx || hy == ty {
                        move_type_simple = true;
                    } else {
                        move_type_simple = false;
                    }
                    for a in move_map[&move_type_simple] {
                        if is_touching(hx, hy, tx + a[0], ty + a[1]) {
                            rope[r][0] += a[0];
                            rope[r][1] += a[1];
                            break;
                        }
                    }
                    if r == 1 {
                        moves1.insert(format!("({},{})", rope[r][0], rope[r][1]));
                    } else if r == 9 {
                        moves9.insert(format!("({},{})", rope[r][0], rope[r][1]));
                    }
                }
            }
        }
    }
    println!("answer 1 => {}", moves1.len());
    println!("answer 2 => {}", moves9.len());
}
fn is_touching(hx: i32, hy: i32, tx: i32, ty: i32) -> bool {
    let moves = [
        //overlap
        [0 + 0, 0 + 0],
        //touching
        [0 + 0, 0 + 1],
        [0 + 0, 0 - 1],
        [0 + 1, 0 + 0],
        [0 - 1, 0 + 0],
        //diagonal
        //[-1, -1], [-1, 1], [1, -1], [1, 1]
        [0 - 1, 0 - 1],
        [0 - 1, 0 + 1],
        [0 + 1, 0 - 1],
        [0 + 1, 0 + 1],
    ];
    moves.iter().any(|x| hx == tx + x[0] && hy == ty + x[1])
}
