use std::fs;

struct Marble {
    value: usize,
    prev_idx: usize, // makeshift linked list LOL
    next_idx: usize,
}

struct Ring {
    marbles: Vec<Marble>,
    current_marble: usize,
}

impl Ring {
    fn new(size: usize) -> Ring {
        let mut marbles = Vec::with_capacity(size);
        marbles.push(Marble {
            value: 0,
            prev_idx: 0,
            next_idx: 0,
        });
        Ring {
            marbles,
            current_marble: 0,
        }
    }

    fn insert(&mut self, value: usize) -> usize {
        let mut score = 0;
        if value % 23 == 0 {
            score += value;
            for _ in 0..7 {
                self.current_marble = self.marbles[self.current_marble].prev_idx;
            }
            score += self.rm_current();
        } else {
            let left_marble = self.marbles[self.current_marble].next_idx;
            let right_marble = self.marbles[left_marble].next_idx;
            self.marbles.push(Marble {
                value,
                prev_idx: left_marble,
                next_idx: right_marble,
            });
            self.marbles[left_marble].next_idx = self.marbles.len() - 1;
            self.marbles[right_marble].prev_idx = self.marbles.len() - 1;
            self.current_marble = self.marbles.len() - 1;
        }
        score
    }

    fn rm_current(&mut self) -> usize {
        let prev = self.marbles[self.current_marble].prev_idx;
        let next = self.marbles[self.current_marble].next_idx;
        self.marbles[prev].next_idx = next;
        self.marbles[next].prev_idx = prev;
        let value = self.marbles[self.current_marble].value;
        self.current_marble = next;
        value
    }
}

fn read_input() -> (usize, usize) {
    let input = fs::read_to_string("inputs/9.txt")
        .expect("Cannot read file")
        .trim()
        .to_string();
    let mut iter = input.split_whitespace().flat_map(|x| x.parse());
    (iter.next().unwrap(), iter.next().unwrap())
}

pub fn day9a() -> usize {
    let (players, last_marble) = read_input();
    play_game(players, last_marble)
}

pub fn day9b() -> usize {
    let (players, last_marble) = read_input();
    play_game(players, last_marble * 100)
}

fn play_game(players: usize, last_marble: usize) -> usize {
    let mut ring = Ring::new(last_marble + 1);
    let mut scores = vec![0; players];
    for (i, value) in (1..=last_marble).enumerate() {
        let player = i % players;
        scores[player] += ring.insert(value);
    }
    *scores.iter().max().unwrap()
}
