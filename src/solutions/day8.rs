use std::fs;

#[derive(Debug, Default)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>,
}

impl Node {
    fn read(nums: &mut impl Iterator<Item = i32>) -> Option<Node> {
        let n_children = nums.next()?;
        let n_metadata = nums.next()?;
        let mut children = Vec::new();
        for _ in 0..n_children {
            children.push(Node::read(nums)?);
        }
        let metadata: Vec<i32> = nums.take(n_metadata as usize).collect();
        Some(Node { children, metadata })
    }

    fn sum_metadata(&self) -> i32 {
        let children_sum: i32 = self.children.iter().map(|c| c.sum_metadata()).sum();
        let metadata_sum: i32 = self.metadata.iter().sum();
        children_sum + metadata_sum
    }

    fn value2(&self) -> i32 {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .map(|&i| {
                    if i == 0 {
                        0
                    } else {
                        self.children.get(i as usize - 1).map_or(0, |c| c.value2())
                    }
                })
                .sum()
        }
    }
}

pub fn day8a() -> i32 {
    let root = read_input();
    root.sum_metadata()
}

pub fn day8b() -> i32 {
    let root = read_input();
    root.value2()
}

fn read_input() -> Node {
    let input = fs::read_to_string("inputs/8.txt")
        .expect("Cannot read file")
        .trim()
        .to_string();
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    Node::read(&mut nums.iter().cloned()).unwrap()
}
