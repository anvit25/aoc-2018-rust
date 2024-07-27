use std::fs::{self, File};
use std::error::Error;
use reqwest::blocking;
use std::io::Write;
use std::path::Path;
// use paste::paste;


pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;


pub fn run_day(day: u8) -> Result<(), Box<dyn Error>> {
    match day {
        1 => {
            download_input(day)?;
            println!("Day 1a: {}, Day 1b: {}", day1::day1a(), day1::day1b());
            Ok(())
        },
        2 => {
            download_input(day)?;
            println!("Day 2a: {}, Day 2b: {}", day2::day2a(), day2::day2b());
            Ok(())
        },
        3 => {
            download_input(day)?;
            println!("Day 3a: {}, Day 3b: {}", day3::day3a(), day3::day3b());
            Ok(())
        },
        4 => {
            download_input(day)?;
            println!("Day 4a: {}, Day 4b: {}", day4::day4a(), day4::day4b());
            Ok(())
        },
        5 => {
            download_input(day)?;
            println!("Day 5a: {}, Day 5b: {}", day5::day5a(), day5::day5b());
            Ok(())
        },
        _ => Err("Day not implemented".into())
    }
}

pub fn run_all() -> Result<(), Box<dyn Error>> {
    for day in 1..5 {
        run_day(day)?;
    }
    Ok(())
}

fn download_input(day: u8) -> Result<String, Box<dyn Error>> {
    // read from the AOC_SESSION_COOKIE.PVT file

    if Path::exists(Path::new(&format!("inputs/{}.txt", day))) {
        return Ok(format!("Input for Day {day} already exists"))
    }
    
    let session_cookie = fs::read_to_string("AOC_SESSION_COOKIE.pvt")?;
    let url = format!("https://adventofcode.com/2018/day/{}/input", day);
    let client = blocking::Client::new();
    let res = client.get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?
        .text()?;

    let mut file = File::create(format!("inputs/{}.txt", day))?;
    file.write_all(res.as_bytes())?;
    Ok(format!("Input for Day {day} downloaded successfully."))
}