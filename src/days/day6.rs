use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day6;

struct Parsed {
    races: Vec<Race>,
}

struct Race {
    duration: u64,
    record: u64,
}

fn parse_a(input: &[String]) -> Parsed {
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
    Parsed { races }
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

impl Solution for Day6 {
    fn part_a(&self, input: &[String]) -> Answer {
        let parsed = parse_a(input);
        parsed
            .races
            .iter()
            .map(|race| {
                (0..race.duration)
                    .filter(|e| e * (race.duration - e) > race.record)
                    .count() as u32
            })
            .product::<u32>()
            .into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let race = parse_b(&input);
        let count = (0..race.duration)
            .filter(|e| e * (race.duration - e) > race.record)
            .count() as u32;
        count.into()
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
