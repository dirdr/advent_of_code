use crate::helper_lib::solution::Solution;

pub struct Day6;

struct Parsed {
    races: Vec<Race>,
}

struct Race {
    duration: u32,
    record: u32,
}

fn parse(input: &[String]) -> Parsed {
    let (_, times) = input[0].split_once("Time:").unwrap();
    let (_, records) = input[1].split_once("Distance:").unwrap();
    let times: Vec<u32> = times
        .split_whitespace()
        .map(|t| t.parse::<u32>().unwrap())
        .collect();
    let records: Vec<u32> = records
        .split_whitespace()
        .map(|r| r.parse::<u32>().unwrap())
        .collect();
    let mut races = vec![];
    for (&duration, &record) in times.iter().zip(&records) {
        races.push(Race { duration, record });
    }
    Parsed { races }
}

impl Solution for Day6 {
    fn part_a(&self, input: &[String]) -> crate::helper_lib::answer::Answer {
        let parsed = parse(input);
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

    fn part_b(&self, input: &[String]) -> crate::helper_lib::answer::Answer {
        todo!()
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
        assert_eq!(<i32 as Into<Answer>>::into(288i32), answer);
    }

    #[test]
    pub fn test_b() {
        let input =
            input::read_file(&format!("{}day_6_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day6.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(46i32), answer);
    }
}
