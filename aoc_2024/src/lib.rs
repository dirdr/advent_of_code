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
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;

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
    &day11::Day11,
    &day12::Day12,
    &day13::Day13,
    &day14::Day14,
    &day15::Day15,
    &day16::Day16,
    &day17::Day17,
    &day18::Day18,
    &day19::Day19,
    &day20::Day20,
    &day21::Day21,
    &day22::Day22,
];

pub const FILES_PREFIX_TEST: &str = "resources/test/";
