use aoc_lib::{
    answer::Answer,
    directions::{Advance, Direction, ExtendedCardinal},
    matrix::Matrix,
    solution::Solution,
    vec2::Vec2,
};

pub struct Day4;

impl Solution for Day4 {
    fn part_a(&self, input: &[String]) -> Answer {
        let matrix = Matrix::from_chars(input);
        let mut count = 0;
        for y in 0..matrix.rows {
            for x in 0..matrix.cols {
                let start = Vec2::new(x, y);
                if matrix[start] != 'X' {
                    continue;
                }
                'dir: for dir in ExtendedCardinal::all_clockwise() {
                    let mut pos = Vec2::<isize>::from(start);
                    for expected in ['M', 'A', 'S'] {
                        let next = dir.advance(pos);
                        if Some(&expected) != matrix.get(&next) {
                            continue 'dir;
                        }
                        pos = next;
                    }
                    count += 1;
                }
            }
        }
        count.into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let matrix = Matrix::from_chars(input);
        let mut count = 0;
        for y in 0..matrix.rows {
            for x in 0..matrix.cols {
                let start = Vec2::new(x, y);
                if matrix[start] != 'A' {
                    continue;
                }
                if is_valid_xmas_pattern(&matrix, &start) {
                    count += 1;
                }
            }
        }
        count.into()
    }
}

fn is_valid_xmas_pattern(matrix: &Matrix<char>, pos: &Vec2<usize>) -> bool {
    let pos = Vec2::<isize>::from(pos);
    let dirs: [[ExtendedCardinal; 2]; 2] = [
        [ExtendedCardinal::NorthWest, ExtendedCardinal::SouthEast],
        [ExtendedCardinal::SouthWest, ExtendedCardinal::NorthEast],
    ];
    for diag in dirs {
        let (mut m, mut s) = (false, false);
        for dir in diag {
            let Some(&char) = matrix.get(&dir.advance(pos)) else {
                return false;
            };
            m ^= char == 'M';
            s ^= char == 'S';
        }
        if !(m && s) {
            return false;
        }
    }
    true
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
        assert_eq!(<i32 as Into<Answer>>::into(9), answer);
    }
}
