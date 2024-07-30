struct Unit {
    x: i32,
    y: i32,
    hp: i32,
    attack: i32,
    elf: bool,
    turn_done: bool,
}

impl Unit {
    fn new(x: i32, y: i32, elf: bool) -> Unit {
        Unit {
            x,
            y,
            hp: 200,
            attack: 3,
            elf,
            turn_done: false,
        }
    }
    fn attack(&self, other: &mut Unit) {
        other.hp -= self.attack;
    }
    fn is_dead(&self) -> bool {
        self.hp <= 0
    }
}

struct Dungeon {
    map: Vec<Vec<char>>,
    units: Vec<Unit>,
}

impl Dungeon {
    fn read(input: Vec<Vec<char>>) -> Dungeon {
        let mut units = Vec::new();
        let mut map = input.clone();
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                match input[y][x] {
                    'E' => units.push(Unit {
                        x: x as i32,
                        y: y as i32,
                        hp: 200,
                        attack: 3,
                        elf: true,
                        turn_done: false,
                    }),
                    'G' => units.push(Unit {
                        x: x as i32,
                        y: y as i32,
                        hp: 200,
                        attack: 3,
                        elf: false,
                        turn_done: false,
                    }),
                    _ => (),
                }
            }
        }
        Dungeon { map, units }
    }

    fn turn(&mut self) {
        self.units.sort_by_key(|unit| (unit.y, unit.x));
        let next_unit = self.units.iter_mut().find(|unit| !unit.turn_done);
        if next_unit.is_none() {
            for unit in self.units.iter_mut() {
                if !unit.is_dead() {
                    unit.turn_done = false;
                }
            }
            return;
        }
        let next_unit = next_unit.unwrap();
        // let mut enemies: Vec<&Unit> = self.units.iter().filter(|unit| unit.elf != next_unit.elf && !unit.is_dead()).collect();
    }
}

pub fn day15a() -> i32 {
    println!("Day 15a: {:?}", read_input());
    todo!()
}

pub fn day15b() -> i32 {
    todo!()
}

fn read_input() -> Vec<Vec<char>> {
    std::fs::read_to_string("inputs/15.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
