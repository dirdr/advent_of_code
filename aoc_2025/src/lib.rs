use aoc_lib::solution::Solution;

pub mod day01;
pub mod day02;
pub mod day03;

pub const ALL: &[&dyn Solution] = &[&day01::Day1, &day02::Day2, &day03::Day3];

pub const FILES_PREFIX_TEST: &str = "resources/test/";
