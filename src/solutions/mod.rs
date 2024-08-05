use aoc_utils::io::download_input;
use std::error::Error;
use std::time::Instant;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day16;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub fn run_day(day: u8) -> Result<(), Box<dyn Error>> {
    download_input(day, 2018)?;
    match day {
        1 => println!("Day 1a: {}, Day 1b: {}", day1::day1a(), day1::day1b()),
        2 => println!("Day 2a: {}, Day 2b: {}", day2::day2a(), day2::day2b()),
        3 => println!("Day 3a: {}, Day 3b: {}", day3::day3a(), day3::day3b()),
        4 => println!("Day 4a: {}, Day 4b: {}", day4::day4a(), day4::day4b()),
        5 => println!("Day 5a: {}, Day 5b: {}", day5::day5a(), day5::day5b()),
        6 => println!("Day 6a: {}, Day 6b: {}", day6::day6a(), day6::day6b()),
        7 => println!("Day 7a: {}, Day 7b: {}", day7::day7a(), day7::day7b()),
        8 => println!("Day 8a: {}, Day 8b: {}", day8::day8a(), day8::day8b()),
        9 => println!("Day 9a: {}, Day 9b: {}", day9::day9a(), day9::day9b()),
        10 => println!("Day 10a: {}, Day 10b: {}", day10::day10a(), day10::day10b()),
        11 => println!("Day 11a: {}, Day 11b: {}", day11::day11a(), day11::day11b()),
        12 => println!("Day 12a: {}, Day 12b: {}", day12::day12a(), day12::day12b()),
        13 => println!("Day 13a: {}, Day 13b: {}", day13::day13a(), day13::day13b()),
        14 => println!("Day 14a: {}, Day 14b: {}", day14::day14a(), day14::day14b()),
        16 => println!("Day 16a: {}, Day 16b: {}", day16::day16a(), day16::day16b()),
        _ => {
            return Err("Day not implemented".into());
        }
    };
    Ok(())
}

pub fn run_all() -> Result<(), Box<dyn Error>> {
    for day in 1..17 {
        if day == 15 {
            continue;
        }
        let now = Instant::now();
        run_day(day)?;
        println!("Time: {}ms", now.elapsed().as_millis());
    }
    Ok(())
}
