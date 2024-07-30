use std::fs;

pub fn day1a() -> i32 {
    read_input().into_iter().sum()
}

pub fn day1b() -> i32 {
    let nums = read_input();
    // calculate the cumulative sum
    let mut sum = 0;
    let mut seen = std::collections::HashSet::new();
    seen.insert(sum);

    loop {
        for num in &nums {
            sum += num;
            if seen.contains(&sum) {
                return sum;
            }
            seen.insert(sum);
        }
    }
}

fn read_input() -> Vec<i32> {
    let file = fs::read_to_string("inputs/1.txt").unwrap();
    file.trim()
        .split('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}
