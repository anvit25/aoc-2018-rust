use std::fs;

fn read_input() -> Vec<char> {
    let input = fs::read_to_string("inputs/5.txt").unwrap();
    input.trim().chars().collect()
}

fn react(a: char, b: char) -> bool {
    a != b && a.to_ascii_lowercase() == b.to_ascii_lowercase()
}

fn chain_destroy_at_index(polymer: &mut Vec<char>, index: usize) -> bool {
    if index >= polymer.len() - 1 {
        return false;
    }
    if react(polymer[index], polymer[index + 1]) {
        polymer.remove(index);
        polymer.remove(index);
        if index != 0 {
            chain_destroy_at_index(polymer, index-1);
        }
    }
    true 
}

fn chain_destroy(polymer: &mut Vec<char>) {
    for i in (0..polymer.len()).rev() {
        chain_destroy_at_index(polymer, i);
    }
}

pub fn day5a() -> usize {
    let mut polymer = read_input();
    chain_destroy(&mut polymer);
    polymer.len()
}

pub fn day5b() -> usize {
    let mut min_length = usize::MAX;
    let mut reduced_polymer = read_input();
    chain_destroy(&mut reduced_polymer);

    for c in b'A'..=b'Z' {
        let mut polymer = reduced_polymer.clone();
        polymer.retain(|&x| x != c as char && x != (c + 32) as char);
        chain_destroy(&mut polymer);
        if polymer.len() < min_length {
            min_length = polymer.len();
        }
    }
    min_length
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_destory() {
        let mut polymer = vec!['B', 'a', 'A', 'b', 'B', 'c'];
        chain_destroy_at_index(&mut polymer, 1);
        assert_eq!(polymer, vec!['B', 'c']);
    }
}