use std::fs;

// When you get paid per line of code 😎
#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Intersection {
    Left,
    Straight,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Cart {
    dir: Dir,
    intersection: Intersection,
    position: (i32, i32),
    moved: bool,
}

impl Cart {
    fn new(dir: Dir, position: (i32, i32)) -> Cart {
        Cart {
            dir,
            intersection: Intersection::Left,
            position,
            moved: false,
        }
    }

    fn update_dir(&mut self, track: &Track) {
        match track {
            Track::Vertical | Track::Horizontal => {}
            Track::ForwardSlash => {
                self.dir = match self.dir {
                    Dir::Up => Dir::Right,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Down,
                    Dir::Right => Dir::Up,
                };
            }
            Track::BackwardSlash => {
                self.dir = match self.dir {
                    Dir::Up => Dir::Left,
                    Dir::Down => Dir::Right,
                    Dir::Left => Dir::Up,
                    Dir::Right => Dir::Down,
                };
            }
            Track::Intersection => match self.intersection {
                Intersection::Left => {
                    self.dir = match self.dir {
                        Dir::Up => Dir::Left,
                        Dir::Down => Dir::Right,
                        Dir::Left => Dir::Down,
                        Dir::Right => Dir::Up,
                    };
                    self.intersection = Intersection::Straight;
                }
                Intersection::Straight => {
                    self.intersection = Intersection::Right;
                }
                Intersection::Right => {
                    self.dir = match self.dir {
                        Dir::Up => Dir::Right,
                        Dir::Down => Dir::Left,
                        Dir::Left => Dir::Up,
                        Dir::Right => Dir::Down,
                    };
                    self.intersection = Intersection::Left;
                }
            },
            Track::Empty => panic!("Cart off the rails at {:?}", self.position),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Track {
    Empty,
    Vertical,
    Horizontal,
    ForwardSlash,
    BackwardSlash,
    Intersection,
}

impl Track {
    fn from_char(c: char) -> Track {
        match c {
            ' ' => Track::Empty,
            '|' => Track::Vertical,
            '-' => Track::Horizontal,
            '/' => Track::ForwardSlash,
            '\\' => Track::BackwardSlash,
            '+' => Track::Intersection,
            '>' | '<' => Track::Horizontal,
            '^' | 'v' => Track::Vertical,
            _ => panic!("Invalid track character"),
        }
    }
}

#[derive(Debug, Clone)]
struct World {
    carts: Vec<Cart>,
    tracks: Vec<Vec<Track>>,
}

impl World {
    fn new(carts: Vec<Cart>, tracks: Vec<Vec<Track>>) -> World {
        World { carts, tracks }
    }

    fn tick(&mut self, part1: bool) -> Option<(i32, i32)> {
        if self.carts.len() == 1 && self.carts[0].moved {
            // println!("{} cars remaining", self.carts.len());
            return Some(self.carts[0].position);
        }

        self.carts
            .sort_by_key(|cart| (cart.position.1, cart.position.0));
        let forbidden_positions: Vec<(i32, i32)> =
            self.carts.iter().map(|cart| cart.position).collect();

        let curr_cart = self.carts.iter_mut().find(|cart| !cart.moved);

        if curr_cart.is_some() {
            let curr_cart = curr_cart.unwrap();
            let (mut x, mut y) = curr_cart.position;
            match curr_cart.dir {
                Dir::Up => y -= 1,
                Dir::Down => y += 1,
                Dir::Left => x -= 1,
                Dir::Right => x += 1,
            }
            curr_cart.position = (x, y);
            if forbidden_positions.contains(&curr_cart.position) {
                if part1 {
                    return Some(curr_cart.position);
                }
                self.carts.retain(|cart| cart.position != (x, y));
                // println!("Crash at {:?}, Remaining Cars: {}", (x, y), self.carts.len());
            } else {
                let track = &self.tracks[y as usize][x as usize];
                curr_cart.update_dir(track);
                curr_cart.moved = true;
            }
        } else {
            for cart in self.carts.iter_mut() {
                cart.moved = false;
            }
            // println!("Carts: {:?}", self.carts);
        }
        None
    }
}

fn read_input() -> Vec<Vec<char>> {
    fs::read_to_string("inputs/13.txt")
        .expect("Cannot read file")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn parse_input(input: Vec<Vec<char>>) -> World {
    let mut carts = Vec::new();
    let mut tracks = Vec::new();

    for (y, line) in input.iter().enumerate() {
        let mut track_line = Vec::new();
        for (x, c) in line.iter().enumerate() {
            let x = x as i32;
            let y = y as i32;
            match c {
                '>' => carts.push(Cart::new(Dir::Right, (x, y))),
                '<' => carts.push(Cart::new(Dir::Left, (x, y))),
                '^' => carts.push(Cart::new(Dir::Up, (x, y))),
                'v' => carts.push(Cart::new(Dir::Down, (x, y))),
                _ => {}
            }
            track_line.push(Track::from_char(*c));
        }
        tracks.push(track_line);
    }
    World::new(carts, tracks)
}

pub fn day13a() -> String {
    let mut world = parse_input(read_input());
    loop {
        if let Some(position) = world.tick(true) {
            return format!("{},{}", position.0, position.1);
        }
    }
}

pub fn day13b() -> String {
    // return "Not Implemented".to_string();
    let mut world = parse_input(read_input());
    loop {
        if let Some(position) = world.tick(false) {
            return format!("{},{}", position.0, position.1);
        }
    }
}
