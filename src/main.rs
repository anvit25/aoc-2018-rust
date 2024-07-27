use aoc_rust_2018::solutions::*;
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    // run_all()?;
    run_day(5)?;
    println!("Time: {}ms", now.elapsed().as_millis());
    Ok(())
}