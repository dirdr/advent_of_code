use aoc_lib::{answer::Answer, solution::Solution};

pub struct Day12;

impl Solution for Day12 {
    fn part_a(&self, input: &[String]) -> Answer {
        let regions = parse(input);
        regions.into_iter().filter(|r| r.is_valid()).count().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        Answer::Unimplemented
    }
}

struct Region {
    area: usize,
    requirement: Vec<usize>,
}

fn parse(input: &[String]) -> Vec<Region> {
    let block = input
        .split(|line| line.is_empty())
        .find(|&l| l.iter().any(|c| c.contains('x')))
        .unwrap();

    block
        .iter()
        .map(|line| Region::from_input_line(line))
        .collect::<Vec<_>>()
}

impl Region {
    fn from_input_line(input_line: &str) -> Self {
        let (area, requirement) = input_line.split_once(':').unwrap();
        let (w, h) = area.split_once('x').unwrap();
        let (w, h) = (w.parse::<usize>().unwrap(), h.parse::<usize>().unwrap());
        let requirement = requirement
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Self {
            area: w * h,
            requirement,
        }
    }

    fn is_valid(&self) -> bool {
        self.requirement.iter().map(|b| b * 9).sum::<usize>() <= self.area
    }
}

#[cfg(test)]
mod test {
    use super::Day12;
    use aoc_lib::{answer::Answer, input, solution::Solution};

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_12_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        assert_eq!(<i64 as Into<Answer>>::into(1), Day12.part_a(&input));
    }
}
