use std::fs;

#[derive(Debug)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32
}

impl Claim {
    fn from_string(s: &str) -> Claim {
        fn is_not_num(c: char) -> bool {
            !c.is_numeric()
        }
        let parts: Vec<&str> = s.split(is_not_num).collect();
        let id = parts[1].parse::<u32>().unwrap();
        let x = parts[4].parse::<u32>().unwrap();
        let y = parts[5].parse::<u32>().unwrap();
        let width = parts[7].parse::<u32>().unwrap();
        let height = parts[8].parse::<u32>().unwrap();
        Claim { id, x, y, width, height }
    }

    fn max_x(&self) -> u32 {
        self.x + self.width
    }

    fn max_y(&self) -> u32 {
        self.y + self.height
    }
}

fn make_fabric_claim_matrix(input: &Vec<Claim>) -> Vec<Vec<u32>> {
    let max_x = input.iter().map(|x| x.max_x()).max().unwrap();
    let max_y = input.iter().map(|x| x.max_y()).max().unwrap();
    let mut fabric = vec![vec![0; max_y as usize]; max_x as usize];
    for claim in input {
        for x in claim.x..claim.max_x() {
            for y in claim.y..claim.max_y() {
                fabric[x as usize][y as usize] += 1;
            }
        }
    }
    fabric
}

pub fn day3a() -> u32 {
    let input = read_input();
    let fabric = make_fabric_claim_matrix(&input);

    let mut count = 0;
    for row in fabric {
        for cell in row {
            if cell > 1 {
                count += 1;
            }
        }
    }

    count
}

pub fn day3b() -> u32 {
    let input = read_input();
    let fabric = make_fabric_claim_matrix(&input);

    for claim in input {
        let mut overlap = false;
        for x in claim.x..claim.max_x() {
            for y in claim.y..claim.max_y() {
                if fabric[x as usize][y as usize] > 1 {
                    overlap = true;
                    break;
                }
            }
            if overlap {
                break;
            }
        }
        if !overlap {
            return claim.id;
        }
    }

    0
}

fn read_input() -> Vec<Claim> {
    let file = fs::read_to_string("inputs/3.txt").unwrap();
    file.trim().split('\n').map(Claim::from_string).collect()
}