use std::collections::HashMap;

use aoc_lib::{answer::Answer, solution::Solution};

pub struct Day1;

impl Solution for Day1 {
    fn part_a(&self, input: &[String]) -> Answer {
        let mut notes = parse(input);
        notes.first.sort();
        notes.second.sort();
        notes
            .first
            .iter()
            .zip(notes.second.iter())
            .map(|(a, b)| (a - b).abs())
            .sum::<i32>()
            .into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let notes = parse(input);
        let mut map: HashMap<i32, i32> = HashMap::new();
        for &el in &notes.second {
            map.entry(el).and_modify(|e| *e += 1).or_insert(1);
        }
        notes
            .first
            .iter()
            .map(|e| *e * map.get(e).unwrap_or(&0))
            .sum::<i32>()
            .into()
    }
}

struct Notes {
    first: Vec<i32>,
    second: Vec<i32>,
}

fn parse(input: &[String]) -> Notes {
    let mut first = vec![];
    let mut second = vec![];
    for line in input {
        let line = line.split_whitespace().collect::<Vec<_>>();
        first.push(line[0].parse::<i32>().unwrap());
        second.push(line[1].parse::<i32>().unwrap());
    }
    Notes { first, second }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day1;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_01_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day1.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(11), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_01_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day1.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(31), answer);
    }
}
