use crate::helper_lib::answer::Answer;
use crate::helper_lib::solution::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn part_a(&self, input: &[String]) -> Answer {
        let result: u32 = input
            .iter()
            .map(|l| {
                let mut digits = l.chars().filter_map(|c| c.to_digit(10));
                let first = digits.next().unwrap();
                let last = digits.last().unwrap_or(first);
                first * 10 + last
            })
            .sum();
        result.into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let result: u32 = input
            .iter()
            .map(|l| {
                let digits = get_digits(l);
                digits[0] * 10 + digits[1]
            })
            .sum();
        result.into()
    }
}

fn get_digits(i: &str) -> [u32; 2] {
    let helper = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut first = None;
    let mut last = 0;

    let mut digit = |c| {
        first = first.or(Some(c));
        last = c;
    };

    let chars = i.as_bytes();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c.is_ascii_digit() {
            digit((c - b'0') as u32);
        } else {
            for (j, d) in helper.iter().enumerate() {
                if chars[i..].starts_with(d.as_bytes()) {
                    digit(j as u32 + 1);
                }
            }
        }
        i += 1;
    }

    [first.unwrap(), last]
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day1;

    #[test]
    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_01_a_test.txt",
            helper_lib::consts::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day1.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(142), answer);
    }

    #[test]
    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_01_b_test.txt",
            helper_lib::consts::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day1.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(281), answer);
    }
}
