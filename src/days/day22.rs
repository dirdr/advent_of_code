use std::path::PrefixComponent;

use num::ToPrimitive;

use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day22;

impl Solution for Day22 {
    fn part_a(&self, input: &[String]) -> Answer {
        let cube = parse(input);
        todo!()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        todo!()
    }
}

fn parse(input: &[String]) -> Cube {
    for line in input {
        let (start, end) = line.split_once('~').unwrap();
        let start = start
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let end = end.split(',').map(|c| c.parse::<usize>().unwrap());
        println!("{:?}", start);
        println!("{:?}", end);
    }
    todo!()
}

struct Cube {
    unit: Vec<Vec<Vec<usize>>>,
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day22;

    #[test]
    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_22_test.txt",
            helper_lib::consts::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day22.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(42), answer);
    }

    #[test]
    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_22_test.txt",
            helper_lib::consts::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day22.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(16733044), answer);
    }
}
