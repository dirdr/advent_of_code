use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day20;

impl Solution for Day20 {
    fn part_a(&self, input: &[String]) -> Answer {
        todo!()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day20;

    #[test]
    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_20_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day20.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(19114), answer);
    }

    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_20_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day20.part_b(&input);
        assert_eq!(<i64 as Into<Answer>>::into(167409079868000), answer);
    }
}
