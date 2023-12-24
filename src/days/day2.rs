use std::{cmp::max, collections::HashMap};

use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day2;

impl Solution for Day2 {
    fn part_a(&self, lines: &[String]) -> Answer {
        let mut sum = 0;
        let colors = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
        for line in lines {
            if let Some(iter) = line.strip_prefix("Game ") {
                let (game_id_str, colors_str) = iter
                    .split_once(':')
                    .ok_or_else(|| anyhow::anyhow!("Invalid line format"))
                    .unwrap();
                let game_id = game_id_str.parse::<i32>().unwrap();
                let valid_game = colors_str.trim().split(';').all(|set| {
                    set.split(',').all(|token| {
                        let tokens = token.split_whitespace().collect::<Vec<&str>>();
                        let val = tokens[0].parse::<i32>().unwrap_or(0);
                        *colors.get(tokens[1]).unwrap_or(&i32::MAX) >= val
                    })
                });
                if valid_game {
                    sum += game_id;
                }
            }
        }
        sum.into()
    }

    fn part_b(&self, lines: &[String]) -> Answer {
        let mut sum = 0;
        for line in lines {
            let mut colors = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
            if let Some(color_data) = line.strip_prefix("Game ").and_then(|s| s.split_once(':')) {
                for set in color_data.1.trim().split(';') {
                    for token in set.split(',') {
                        let tokens = token.split_whitespace().collect::<Vec<&str>>();
                        let val = tokens[0].parse::<i32>().unwrap();
                        colors.entry(tokens[1]).and_modify(|e| *e = max(val, *e));
                    }
                }
            }
            sum += colors.values().product::<i32>();
        }
        sum.into()
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day2;

    #[test]
    pub fn test_a() {
        let input =
            input::read_file(&format!("{}day_2_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day2.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(8), answer);
    }

    #[test]
    pub fn test_b() {
        let input =
            input::read_file(&format!("{}day_2_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day2.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(2286), answer);
    }
}
