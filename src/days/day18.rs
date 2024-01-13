use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day18;

impl Solution for Day18 {
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

    use super::Day18;

    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_18_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day18.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(62), answer);
    }

    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_18_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day18.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(0), answer);
    }
}
