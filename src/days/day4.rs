use crate::helper_lib::answer::Answer;
use crate::helper_lib::solution::Solution;

pub struct Day4;

impl Solution for Day4 {
    fn part_a(&self, lines: &[String]) -> Answer {
        let mut sum = 0;
        for line in lines {
            let line = line.split_once(":").unwrap().1.trim();
            let (winning, pick) = line.split_once("|").unwrap();
            let winning = convert_to_nums(winning);
            let pick = convert_to_nums(pick);
            sum += 2_i32.pow((pick.iter().filter(|num| winning.contains(num)).count() - 1) as u32);
        }
        sum.into()
    }

    fn part_b(&self, lines: &[String]) -> Answer {
        // we start with one copy of each of the cards,
        // cards are 0 indexed : 1st card : copies[0], ...
        let mut copies: Vec<usize> = vec![1; lines.len()];
        for (i, &ref line) in lines.iter().enumerate() {
            for _ in 0..copies[i] {
                let line = line.split_once(":").unwrap().1.trim();
                let (winning, pick) = line.split_once("|").unwrap();
                let winning = convert_to_nums(winning);
                let pick = convert_to_nums(pick);
                let winning_numbers = pick.iter().filter(|num| winning.contains(num)).count();
                for k in i..(i + winning_numbers) {
                    copies[k + 1] += 1;
                }
            }
        }
        let sum = copies.iter().sum::<usize>();
        sum.into()
    }
}

fn convert_to_nums(str_set: &str) -> Vec<usize> {
    let mut nums = vec![];
    for token in str_set.split_whitespace() {
        nums.push(token.parse::<usize>().unwrap())
    }
    nums
}
