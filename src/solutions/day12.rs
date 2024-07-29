use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
struct Garden {
    pots: VecDeque<bool>,
    offset: i64,
    rules: HashMap<u8, bool>
}

impl Garden {
    fn read_rule(s: &str) -> (u8, bool) {
        let mut elements = s.split(" => ");
        let pattern: Vec<bool> = elements.next()
                .unwrap().chars()
                .map(|c| c == '#').collect();
        let result = elements.next().unwrap() == "#";
        (bool_vec_u8(&pattern), result)
    }

    fn new(initial_state: String, rules: Vec<(u8, bool)>) -> Garden {
        let mut pots: VecDeque<bool> = initial_state.chars().map(|c| c == '#').collect();
        for _ in 0..4 {
            pots.push_front(false);
            pots.push_back(false);
        }
        Garden { pots, offset: 4, rules: HashMap::from_iter(rules) }
    }

    fn next_generation(&mut self) {
        let mut new_pots = VecDeque::new();
        for i in 2..self.pots.len()-2 {
            let pattern: Vec<bool> = self.pots.iter().skip(i-2).take(5).map(|b| *b).collect();
            let pattern = bool_vec_u8(&pattern);
            let result = self.rules.get(&pattern).unwrap();
            new_pots.push_back(*result);
        }
        self.pots = new_pots;
        self.offset -= 2;
        self.clean_edges();
    }

    fn clean_edges(&mut self) {
        while !self.pots.front().unwrap() {
            self.pots.pop_front();
            self.offset -= 1;
        }
        while !self.pots.back().unwrap() {
            self.pots.pop_back();
        }
        for _ in 0..4 {
            self.pots.push_front(false);
            self.pots.push_back(false);
        }
        self.offset += 4;
    }

    fn sum(&self) -> i64 {
        let mut ans = self.pots.iter().enumerate().map(|(i, b)| if *b {i as i64} else { 0 }).sum();
        ans -= self.offset * self.pots.iter().filter(|b| **b).count() as i64;
        ans
    }
}

impl std::fmt::Display for Garden {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let pots: String = self.pots.iter().map(|b| if *b { '#' } else { '.' }).collect();
        write!(f, "{} {}", self.offset, pots)
    }
}

fn bool_vec_u8(bools: &[bool]) -> u8 {
    let mut ans = 0;
    for i in bools {
        ans = (ans << 1) + *i as u8
    }
    ans
}

fn read_input() -> Garden {
    let input = std::fs::read_to_string("inputs/12.txt")
                        .expect("Cannot read file");
    let mut lines = input.lines();
    let initial_state = lines.next().unwrap().split(": ").nth(1).unwrap().to_string();
    let _ = lines.next();
    let rules = lines.map(Garden::read_rule).collect();
    Garden::new(initial_state, rules)
}

pub fn day12a() -> i64 {
    let mut garden = read_input();
    for _ in 0..20 {
        garden.next_generation();
    }
    garden.sum()
}

pub fn day12b() -> i64 {
    let n_gens = 50_000_000_000_i64;
    let mut garden = read_input();
    
    for _ in 0..200 {
        garden.next_generation();
    }
    let delta = find_diff(&mut garden.clone());
    let ans = garden.sum() + delta * (n_gens - 200);
    ans
}

fn find_diff(garden: &mut Garden) -> i64 {
    // Based on the assumption that it will eventually be an AP

    for _ in 1..=200 {
        garden.next_generation();
    }
    let old_sum = garden.sum();
    garden.next_generation();
    let new_sum = garden.sum();
    new_sum - old_sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_vec_u8() {
        assert_eq!(bool_vec_u8(&vec![true, false, false]), 4);
        assert_eq!(bool_vec_u8(&vec![false, true, true]), 3);
    }

    #[test]
    fn test_rule_from_str(){
        let (num, res) = Garden::read_rule("..#.# => #"); 
        assert_eq!(num, 5);
        assert!(res);
    }
}