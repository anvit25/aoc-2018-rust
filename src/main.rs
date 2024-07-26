use aoc_rust_2018::solutions::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let ans = run_day(10)?;
    println!("{}", ans);
    Ok(())
}