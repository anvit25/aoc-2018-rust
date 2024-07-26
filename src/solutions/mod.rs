use std::fs::{self, File};
use std::error::Error;
use reqwest::blocking;
use std::io::Write;
use std::path::Path;

pub mod day1;

pub fn run_day(day: u8) -> Result<i64, Box<dyn Error>> {
    match day {
        1 => {
            download_input(day)?;
            Ok(day1::day1a() as i64)
        }
        _ => Err("Day not implemented".into())
    }
}

fn download_input(day: u8) -> Result<String, Box<dyn Error>> {
    // read from the AOC_SESSION_COOKIE.PVT file

    if Path::exists(Path::new(&format!("inputs/{}.txt", day))) {
        return Ok(format!("Input for Day {day} already exists"))
    }
    
    let session_cookie = fs::read_to_string("AOC_SESSION_COOKIE.pvt")?;
    let url = format!("https://adventofcode.com/2018/day/{}/input", day);
    let client = blocking::Client::new();
    let res = client.get(&url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?
        .text()?;

    let mut file = File::create(format!("inputs/{}.txt", day))?;
    file.write_all(res.as_bytes())?;
    Ok(format!("Input for Day {day} downloaded successfully."))
}