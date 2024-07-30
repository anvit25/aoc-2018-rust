struct SummedAreaTable {
    table: Vec<Vec<i32>>,
}

impl SummedAreaTable {
    // This is O(n^2)
    fn new(grid: Vec<Vec<i32>>) -> SummedAreaTable {
        let mut table: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];

        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                let mut value = grid[x][y];
                if x > 0 {
                    value += table[x - 1][y];
                }
                if y > 0 {
                    value += table[x][y - 1];
                }
                if x > 0 && y > 0 {
                    value -= table[x - 1][y - 1];
                }
                table[x][y] = value;
            }
        }
        SummedAreaTable { table }
    }

    // This is O(1)
    fn get(&self, x: usize, y: usize, size: usize) -> i32 {
        let mut value = self.table[x + size - 1][y + size - 1];
        if x > 0 {
            value -= self.table[x - 1][y + size - 1];
        }
        if y > 0 {
            value -= self.table[x + size - 1][y - 1];
        }
        if x > 0 && y > 0 {
            value += self.table[x - 1][y - 1];
        }
        value
    }
}

pub fn day11a() -> String {
    let serial_number = read_input();
    let grid = generate_grid(serial_number);
    let table = SummedAreaTable::new(grid);

    let mut max_power = i32::MIN;
    let mut best_point = (0, 0);

    // This is O(n^2)
    for x in 0..300 - 3 {
        for y in 0..300 - 3 {
            let power = table.get(x, y, 3);
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
    let table = SummedAreaTable::new(grid);

    let mut max_power = i32::MIN;
    let mut best_point = (0, 0);
    let mut best_size = 0;

    // This is O(n^3)
    for size in 1..=300 {
        for x in 0..300 - size {
            for y in 0..300 - size {
                let power = table.get(x, y, size);
                if power > max_power {
                    max_power = power;
                    best_point = (x, y);
                    best_size = size;
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
    fn test_sum_area_table() {
        let grid = vec![
            vec![31, 2, 4, 33, 5, 36],
            vec![12, 26, 9, 10, 29, 25],
            vec![13, 17, 21, 22, 20, 18],
            vec![24, 23, 15, 16, 14, 19],
            vec![30, 8, 28, 27, 11, 7],
            vec![1, 35, 34, 3, 32, 6],
        ];
        let table = SummedAreaTable::new(grid);
        assert_eq!(table.table[0][0], 31);
        assert_eq!(table.table[0][1], 33);
        assert_eq!(table.table[1][0], 43);
        assert_eq!(table.table[1][1], 71);
        assert_eq!(table.table[2][2], 31 + 2 + 4 + 12 + 26 + 9 + 13 + 17 + 21);

        assert_eq!(table.get(0, 0, 2), 71);
        assert_eq!(table.get(1, 1, 2), 26 + 9 + 17 + 21);
        assert_eq!(
            table.get(0, 3, 3),
            33 + 5 + 36 + 10 + 29 + 25 + 22 + 20 + 18
        );
    }
}
