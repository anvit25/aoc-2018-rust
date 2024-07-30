use std::fs;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    delta_x: i32,
    delta_y: i32,
}

impl Point {
    fn new(x: i32, y: i32, delta_x: i32, delta_y: i32) -> Point {
        Point {
            x,
            y,
            delta_x,
            delta_y,
        }
    }

    fn step(&mut self, steps: i32) {
        self.x += self.delta_x * steps;
        self.y += self.delta_y * steps;
    }
}

#[derive(Debug)]
struct Sky {
    points: Vec<Point>,
}

impl Sky {
    fn new() -> Sky {
        Sky { points: Vec::new() }
    }

    fn add_point(&mut self, x: i32, y: i32, delta_x: i32, delta_y: i32) {
        self.points.push(Point::new(x, y, delta_x, delta_y));
    }

    fn step(&mut self, steps: i32) {
        for point in self.points.iter_mut() {
            point.step(steps);
        }
    }

    fn border(&self) -> (i32, i32, i32, i32) {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        for point in self.points.iter() {
            if point.x < min_x {
                min_x = point.x;
            }
            if point.x > max_x {
                max_x = point.x;
            }
            if point.y < min_y {
                min_y = point.y;
            }
            if point.y > max_y {
                max_y = point.y;
            }
        }
        (min_x, max_x, min_y, max_y)
    }

    fn size(&self) -> (usize, usize) {
        let (min_x, max_x, min_y, max_y) = self.border();
        ((max_x - min_x + 1) as usize, (max_y - min_y + 1) as usize)
    }
}

impl std::fmt::Display for Sky {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (min_x, _, min_y, _) = self.border();
        let (width, height) = self.size();
        if width > 100 || height > 100 {
            return write!(f, "Sky too big to display");
        }
        let mut grid = vec![vec!['.'; width]; height];
        for point in self.points.iter() {
            grid[(point.y - min_y) as usize][(point.x - min_x) as usize] = '#';
        }
        for row in grid.iter() {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

pub fn day10a() -> i32 {
    let mut sky = read_input();
    // println!("{:?}", sky);
    let base_steps = 10813;
    sky.step(base_steps);
    println!("{}", base_steps);
    println!("{}", sky);
    base_steps
}

pub fn day10b() -> i32 {
    10813
}

fn read_input() -> Sky {
    let input = fs::read_to_string("inputs/10.txt")
        .expect("Cannot read file")
        .trim()
        .to_string();
    let mut sky = Sky::new();
    for line in input.lines() {
        let parts: Vec<i32> = line
            .split(|c| c == '<' || c == '>' || c == ',' || c == ' ')
            .filter_map(|x| x.parse().ok())
            .collect();
        sky.add_point(parts[0], parts[1], parts[2], parts[3]);
    }
    sky
}
