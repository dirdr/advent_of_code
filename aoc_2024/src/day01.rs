use aoc_lib::{answer::Answer, solution::Solution};

pub struct Day1;

impl Solution for Day1 {
    fn part_a(&self, input: &[String]) -> Answer {
        todo!()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day1;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_01_a_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day1.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(142), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_01_b_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day1.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(281), answer);
    }
}
