mod args;
mod days;
mod helper_lib;

use clap::Parser;
use days::lib::ALL;

use crate::helper_lib::input;

#[macro_use]
extern crate anyhow;

fn main() -> anyhow::Result<()> {
    let args = args::Arguments::parse();
    let day = args.day;
    let solution = ALL[day - 1];
    let input = input::read_file(&format!("{}day_{}.txt", helper_lib::FILES_PREFIX, day))?;
    println!("Running Solutions for day {day}");
    println!("Answer for part A : {}", solution.part_a(&input));
    println!("Answer for part B : {}", solution.part_b(&input));
    Ok(())
}
