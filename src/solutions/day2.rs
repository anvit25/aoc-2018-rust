use std::collections::HashMap;
use std::fs;

pub fn day2a() -> i32 {
    let mut twos = 0;
    let mut threes = 0;

    let lines = read_input();
    for word in lines {
        let mut counts = HashMap::new();
        for c in word {
            let count = counts.entry(c).or_insert(0);
            *count += 1;
        }

        if counts.values().any(|&x| x == 2) {
            twos += 1;
        }
        if counts.values().any(|&x| x == 3) {
            threes += 1;
        }
    }

    twos * threes
}

pub fn day2b() -> String {
    let lines = read_input();
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            let mut diff = 0;
            let mut common = String::new();
            for (a, b) in lines[i].iter().zip(lines[j].iter()) {
                if a != b {
                    diff += 1;
                } else {
                    common.push(*a);
                }
            }
            if diff == 1 {
                return common;
            }
        }
    }
    String::new()
}

fn read_input() -> Vec<Vec<char>> {
    let file = fs::read_to_string("inputs/2.txt").unwrap();
    file.trim()
        .split('\n')
        .map(|x| x.chars().collect())
        .collect()
}
