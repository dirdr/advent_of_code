use crate::helper_lib::{
    answer::Answer, directions::Direction, matrix::Matrix, solution::Solution,
};

pub struct Day16;

impl Solution for Day16 {
    fn part_a(&self, input: &[String]) -> Answer {
        let grid = parse(input);
        let (initial_direction, initial_position) = ((0, 0), Direction::East);
        todo!()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        todo!()
    }
}

struct Grid {
    matrix: Matrix<char>,
}

impl Grid {
    fn simulate_beam(&self) -> usize {
        0
    }
}

fn parse(input: &[String]) -> Grid {
    Grid {
        matrix: Matrix::from_chars(input),
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day16;

    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_16_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day16.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(1320), answer);
    }

    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_16_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day16.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(145), answer);
    }
}
