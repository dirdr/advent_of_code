use aoc_lib::solution::Solution;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;

pub const ALL: &[&dyn Solution] = &[
    &day01::Day1,
    &day02::Day2,
    &day03::Day3,
    &day04::Day4,
    &day05::Day5,
    &day06::Day6,
    &day07::Day7,
    &day08::Day8,
    &day09::Day9,
    &day10::Day10,
];

pub const FILES_PREFIX_TEST: &str = "resources/test/";
