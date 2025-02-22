pub mod solutions;
pub mod utils;

use solutions::{run_all, run_day};
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    run_all()?;
    run_day(16)?;
    println!("\nTotal Time: {}ms", now.elapsed().as_millis());
    Ok(())
}
