use aoc_lib::solution::Solution;

pub mod day01;
pub mod day02;

pub const ALL: &[&dyn Solution] = &[&day01::Day1, &day02::Day2];

pub const FILES_PREFIX_TEST: &str = "resources/test/";
