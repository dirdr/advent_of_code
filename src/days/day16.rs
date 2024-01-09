use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day16;

impl Solution for Day16 {
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

    use super::Day16;

    pub fn test_a() {
        let input = input::read_file(&format!(
            "{}day_16_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day16.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(1320), answer);
    }

    pub fn test_b() {
        let input = input::read_file(&format!(
            "{}day_16_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day16.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(145), answer);
    }
}
