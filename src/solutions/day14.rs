struct State {
    recipes: Vec<u8>,
    elf1: usize,
    elf2: usize,
}

impl State {
    fn new(capacity: usize) -> State {
        let mut recipes = Vec::with_capacity(capacity + 11);
        recipes.push(3);
        recipes.push(7);
        State {
            recipes,
            elf1: 0,
            elf2: 1,
        }
    }

    fn tick(&mut self) {
        let sum = self.recipes[self.elf1] + self.recipes[self.elf2];
        if sum >= 10 {
            self.recipes.push(sum / 10);
        }
        self.recipes.push(sum % 10);
        self.elf1 = (self.elf1 + 1 + self.recipes[self.elf1] as usize) % self.recipes.len();
        self.elf2 = (self.elf2 + 1 + self.recipes[self.elf2] as usize) % self.recipes.len();
    }

    fn find(&mut self, pattern: &[u8]) -> usize {
        let n = pattern.len();
        while self.recipes.len() < n + 1 {
            self.tick();
        }
        let mut start = 0;
        let mut end = start + n;
        let mut to_check = &self.recipes[start..end];

        while to_check != pattern {
            if end == self.recipes.len() {
                self.tick();
            }
            start += 1;
            end += 1;
            to_check = &self.recipes[start..end];
        }
        start
    }
}

pub fn day14a() -> String {
    let input = read_input().parse().unwrap();
    let mut state = State::new(input + 11);
    while state.recipes.len() < input + 10 {
        state.tick();
    }
    return state.recipes[input..input + 10]
        .iter()
        .map(|x| x.to_string())
        .collect();
}

pub fn day14b() -> String {
    let input = read_input();
    let input_n: usize = input.parse().unwrap();
    let mut state = State::new(input_n + 11);
    let pattern = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();
    state.find(&pattern).to_string()
}

fn read_input() -> String {
    std::fs::read_to_string("inputs/14.txt")
        .expect("Cannot read file")
        .trim()
        .to_string()
}
