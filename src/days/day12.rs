use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day12;

impl Solution for Day12 {
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

    use super::Day12;

    pub fn test_a() {
        let input =
            input::read_file(&format!("{}day_12_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day12.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(21), answer);
    }

    pub fn test_b() {
        let input =
            input::read_file(&format!("{}day_12_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day12.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(82000210), answer);
    }
}
