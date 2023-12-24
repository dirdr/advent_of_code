use crate::helper_lib::{answer::Answer, solution::Solution};

pub struct Day10;

struct Grid {
    grid: Vec<Vec<Tile>>,
}

struct Tile {
    position: [usize; 2],
    tile: char,
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

const TILES: [(char, [Direction; 2]); 6] = [
    ('|', [Direction::South, Direction::North]),
    ('-', [Direction::West, Direction::East]),
    ('F', [Direction::South, Direction::East]),
    ('7', [Direction::South, Direction::West]),
    ('L', [Direction::North, Direction::East]),
    ('J', [Direction::North, Direction::West]),
];

fn parse(input: &[String]) -> Grid {
    let mut grid = vec![];
    for (i, row) in input.iter().enumerate() {
        let mut row_buffer = vec![];
        for (j, col) in row.chars().enumerate() {
            let tile = Tile {
                position: [i, j],
                tile: col,
            };
            row_buffer.push(tile);
        }
        grid.push(row_buffer);
    }
    Grid { grid }
}

impl Solution for Day10 {
    fn part_a(&self, input: &[String]) -> Answer {
        let grid = parse(input);
        for row in grid.grid {
            for col in row {
                println!("{}", col.tile);
            }
        }
        0.into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day10;

    #[test]
    pub fn test_a() {
        let input =
            input::read_file(&format!("{}day_10_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day10.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(142), answer);
    }

    pub fn test_b() {
        let input =
            input::read_file(&format!("{}day_10_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day10.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(281), answer);
    }
}
