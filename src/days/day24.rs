use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day24;

impl Solution for Day24 {
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

    use super::Day24;

    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_24_test.txt",
            helper_lib::consts::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day24.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(94), answer);
    }

    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_24_test.txt",
            helper_lib::consts::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day24.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(154), answer);
    }
}
