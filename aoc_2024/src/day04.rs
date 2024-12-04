use aoc_lib::{
    answer::Answer,
    directions::{Direction, ExtendedCardinal},
    matrix,
    solution::Solution,
};

pub struct Day4;

impl Solution for Day4 {
    fn part_a(&self, input: &[String]) -> Answer {
        let matrix = matrix::Matrix::from_chars(input);
        for i in 0..matrix.rows {
            for j in 0..matrix.cols {
                for dir in ExtendedCardinal::all_clockwise() {
                    for next in ['M', 'A', 'S'] {}
                }
            }
        }
        todo!()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day4;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_04_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day4.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(18), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_04_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day4.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(4), answer);
    }
}
