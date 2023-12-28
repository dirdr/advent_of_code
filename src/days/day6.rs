use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day6;

impl Solution for Day6 {
    fn part_a(&self, input: &[String]) -> Answer {
        let races = parse_a(input);
        races
            .iter()
            .map(|race| race.count_ways())
            .product::<u32>()
            .into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        parse_b(input).count_ways().into()
    }
}

fn parse_a(input: &[String]) -> Vec<Race> {
    let (_, durations) = input[0].split_once("Time:").unwrap();
    let (_, records) = input[1].split_once("Distance:").unwrap();
    let durations: Vec<u64> = durations
        .split_whitespace()
        .map(|t| t.parse::<u64>().unwrap())
        .collect();
    let records: Vec<u64> = records
        .split_whitespace()
        .map(|r| r.parse::<u64>().unwrap())
        .collect();
    let mut races = vec![];
    for (&duration, &record) in durations.iter().zip(&records) {
        races.push(Race { duration, record });
    }
    races
}

fn parse_b(input: &[String]) -> Race {
    let (_, time) = input[0].split_once("Time:").unwrap();
    let (_, record) = input[1].split_once("Distance:").unwrap();
    let duration: u64 = time
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let record: u64 = record
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    Race { duration, record }
}

struct Race {
    duration: u64,
    record: u64,
}

impl Race {
    pub fn count_ways(&self) -> u32 {
        (0..self.duration)
            .filter(|e| e * (self.duration - e) > self.record)
            .count() as u32
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day6;

    #[test]
    pub fn test_a() {
        let input =
            input::read_file(&format!("{}day_6_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day6.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(288), answer);
    }

    #[test]
    pub fn test_b() {
        let input =
            input::read_file(&format!("{}day_6_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day6.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(71503), answer);
    }
}
