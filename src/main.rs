mod args;

use aoc_lib::{input, solution::Solution};
use args::Args;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let day = args.day;
    let year = args.year;
    let solutions = get_year(year);
    let solution = solutions[day as usize - 1];
    let input_path = &format!(
        "{}/resources/real/day_{:02}.txt",
        get_year_prefix(year),
        day
    );
    println!("{:?} t", input_path.clone());
    let input = input::read_file(input_path)?;
    println!("Running Solutions for day {day}");
    println!("Answer for part A : {}", solution.part_a(&input));
    println!("Answer for part B : {}", solution.part_b(&input));
    Ok(())
}

fn get_year(year: u16) -> &'static [&'static dyn Solution] {
    match year {
        2023 => aoc_2023::ALL,
        2024 => aoc_2024::ALL,
        _ => &[],
    }
}

fn get_year_prefix(year: u16) -> &'static str {
    match year {
        2023 => "aoc_2023",
        2024 => "aoc_2024",
        _ => unimplemented!("Please provide a correct year"),
    }
}
