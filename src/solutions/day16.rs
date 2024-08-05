use std::collections::{HashMap, HashSet};

use super::super::utils::opcode::OpCode;

type Register = [usize; 4];
type Instruction = [usize; 4];
type OpCodeExample = (Register, Instruction, Register);

pub fn day16a() -> usize {
    let (examples, _) = read_input();
    let mut count = 0;
    for ex in examples {
        if test_example(ex).len() >= 3 {
            count += 1;
        }
    }
    count
}

pub fn day16b() -> usize {
    let mut possibilities: HashMap<usize, HashSet<OpCode>> = HashMap::new();
    let (examples, instructions) = read_input();

    for (inp, inst, out) in examples {
        let mut curr_poss = HashSet::new();
        for op in OpCode::ALL.iter() {
            let mut register = inp;
            op.apply(&mut register, inst[1], inst[2], inst[3]);
            if register == out {
                curr_poss.insert(*op);
            }
        }
        // check if inst[0] is in the keys
        if let Some(set) = possibilities.get_mut(&inst[0]) {
            *set = set.intersection(&curr_poss).copied().collect();
        } else {
            possibilities.insert(inst[0], curr_poss);
        }
    }

    let mut fosho: HashMap<usize, OpCode> = HashMap::new();
    while fosho.len() < 16 {
        for (k, v) in possibilities.iter() {
            if v.len() != 1 {
                continue;
            }
            let op = *v.iter().next().unwrap();
            fosho.insert(*k, op);
        }
        for (_, set) in possibilities.iter_mut() {
            for (_, op) in fosho.iter() {
                set.remove(op);
            }
        }
    }

    let mut register = [0, 0, 0, 0];
    for inst in instructions {
        let op = fosho.get(&inst[0]).unwrap();
        op.apply(&mut register, inst[1], inst[2], inst[3]);
    }

    register[0]
}

fn test_example(ex: OpCodeExample) -> Vec<OpCode> {
    let mut count = Vec::new();
    for op in OpCode::ALL.iter() {
        let mut register = ex.0;
        op.apply(&mut register, ex.1[1], ex.1[2], ex.1[3]);
        if register == ex.2 {
            count.push(*op);
        }
    }
    count
}

fn read_input() -> (Vec<OpCodeExample>, Vec<Instruction>) {
    let full: Vec<String> = std::fs::read_to_string("inputs/16.txt")
        .expect("Can't find input file 16.txt")
        .split("\n\n\n")
        .map(|s| s.to_string())
        .collect();

    let first: Vec<OpCodeExample> = full[0].split("\n\n").map(read_part1).collect();

    let second: Vec<Instruction> = full[1].trim().split('\n').map(read_part2).collect();

    (first, second)
}

fn read_part1(inp: &str) -> OpCodeExample {
    let inp: Vec<&str> = inp.trim().split('\n').collect();
    let before: Vec<usize> = inp[0][9..19]
        .split(", ")
        .map(|s| s.parse().unwrap())
        .collect();
    let mid: Vec<usize> = inp[1].split(' ').map(|s| s.parse().unwrap()).collect();
    let after: Vec<usize> = inp[2][9..19]
        .split(", ")
        .map(|s| s.parse().unwrap())
        .collect();

    (
        before.try_into().unwrap(),
        mid.try_into().unwrap(),
        after.try_into().unwrap(),
    )
}

fn read_part2(inp: &str) -> Instruction {
    inp.split(' ')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap()
}
