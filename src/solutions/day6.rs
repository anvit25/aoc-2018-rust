use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn from_str(s: &str) -> Point {
        let mut parts = s.split(", ");
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        Self::new(x, y)
    }

    fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }

    fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

}

fn read_input() -> Vec<Point> {
    let input: Vec<Point> = fs::read_to_string("inputs/6.txt")
                .unwrap().lines().map(Point::from_str).collect();
    input
}


fn border_points(points: &Vec<Point>) -> [i32; 4] {
    let mut min_x = i32::max_value();
    let mut min_y = i32::max_value();
    let mut max_x = i32::min_value();
    let mut max_y = i32::min_value();
    for point in points {
        if point.x < min_x {
            min_x = point.x;
        } else if point.x > max_x {
            max_x = point.x;
        }

        if point.y < min_y {
            min_y = point.y;
        } else if point.y > max_y {
            max_y = point.y;
        }
    }
    [min_x, min_y, max_x, max_y]
}


pub fn day6a() -> usize {
    let points = read_input();
    let [min_x, min_y, max_x, max_y] = border_points(&points);
    let mut coverage: HashMap<&Point, i32> = HashMap::new();
    // let mut grid: Vec<Vec<Option<&Point>>> = vec![vec![None; (max_y - min_y + 1) as usize]; (max_x - min_x + 1) as usize];
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let mut min_distance = i32::max_value();
            let mut closest_point = None;
            for point in points.iter() {
                let distance = point.distance(&Point::new(x,y));
                if distance < min_distance {
                    min_distance = distance;
                    closest_point = Some(point);
                } else if distance == min_distance {
                    closest_point = None;
                }
            }
            if let Some(point) = closest_point {
                let count = coverage.entry(&point).or_insert(0);
                *count += 1;
                if x == min_x || x == max_x || y == min_y || y == max_y {
                    *count = i32::min_value();
                }
            }
        }
    }
    *coverage.values().max().unwrap() as usize
}

pub fn day6b() -> usize {
    let points = read_input();
    let [min_x, min_y, max_x, max_y] = border_points(&points);
    let mut region_size = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let total_distance: i32 = points.iter().map(|point| point.distance(&Point::new(x,y))).sum();
            if total_distance < 10000 {
                region_size += 1;
            }
        }
    }
    region_size
}