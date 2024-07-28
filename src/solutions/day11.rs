pub fn day11a() -> String {
    let serial_number = read_input();
    let grid = generate_grid(serial_number);
    let mut max_power = i32::MIN;
    let mut best_point = (0, 0);

    for x in 0..298 {
        for y in 0..298 {
            let mut power = 0;
            for i in 0..3 {
                for j in 0..3 {
                    power += grid[x + i][y + j];
                }
            }
            if power > max_power {
                max_power = power;
                best_point = (x, y);
            }
        }
    }
    format!("{},{}", best_point.0, best_point.1)
}

pub fn day11b() -> String {
    let serial_number = read_input();
    let grid = generate_grid(serial_number);
    let mut max_power = i32::MIN;
    let mut best_point = (0, 0);
    let mut best_size = 0usize;

    let mut current_size = 1;
    let mut power_levels: Vec<Vec<i32>> = grid.clone();

    while current_size < 300 {
        increase_size(&grid, &mut power_levels, current_size);
        current_size += 1;
        for x in 0..300 - current_size + 1 {
            for y in 0..300 - current_size + 1 {
                let power = power_levels[x][y];
                if power > max_power {
                    max_power = power;
                    best_point = (x, y);
                    best_size = current_size;
                }
            }
        }
    }

    format!("{},{},{}", best_point.0, best_point.1, best_size)
}

fn read_input() -> i32 {
    std::fs::read_to_string("inputs/11.txt")
        .expect("Can't find input file 11")
        .trim()
        .parse()
        .expect("Can't parse input")
}

fn increase_size(grid: &Vec<Vec<i32>>, power_levels: &mut Vec<Vec<i32>>, size: usize) {
    let total_size = grid.len() - size;
    let mut new_power_levels: Vec<Vec<i32>> = Vec::new();
    for x in 0..total_size {
        new_power_levels.push(vec![0; total_size]);
        for y in 0..total_size {
            let mut power = power_levels[x][y];
            for i in 0..size {
                power += grid[x + i as usize][y + size as usize];
            }
            for j in 0..size {
                power += grid[x + size as usize][y + j as usize];
            }
            power += grid[x + size as usize][y + size as usize];
            new_power_levels[x][y] = power;
        }
    }
    *power_levels = new_power_levels;
}

fn power_level(x: i32, y: i32, serial_number: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial_number;
    power_level *= rack_id;
    power_level = (power_level / 100) % 10;
    power_level - 5
}

fn generate_grid(serial_number: i32) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for x in 0..300 {
        grid.push((0..300).map(|y| power_level(x, y, serial_number)).collect());
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn test_increase_size() {
        let grid: Vec<Vec<i32>>;
        grid = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];
        let mut power_levels: Vec<Vec<i32>> = grid.clone();
        println!("{:?}", power_levels);
        increase_size(&grid, &mut power_levels, 1);
        println!("{:?}", power_levels);
        assert_eq!(power_levels.len(), 2);
        assert_eq!(power_levels[0][0], 12);
        assert_eq!(power_levels[0][1], 16);
        assert_eq!(power_levels[1][0], 24);
        assert_eq!(power_levels[1][1], 28);
    }
}