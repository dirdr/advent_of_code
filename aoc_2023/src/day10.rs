use aoc_lib::{
    answer::Answer,
    directions::{Advance, Cardinal, Direction},
    matrix::Matrix,
    solution::Solution,
    vec2::Vec2,
};

pub struct Day10;

impl Solution for Day10 {
    fn part_a(&self, input: &[String]) -> Answer {
        let grid = parse(input);
        (grid.walk().len() / 2).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let grid = parse(input);
        // use of the shoelace polygon area formula
        let mut path = grid.walk();
        path.push(grid.start_tile.position);
        let len = path.len();
        let area = path
            .iter()
            .enumerate()
            .fold(0i32, |acc, (i, p)| {
                let l = (i + 1) % len;
                acc + (p.x as i32 * path[l].y as i32 - p.y as i32 * path[l].x as i32)
            })
            .abs()
            / 2;
        (area - (len as i32 / 2) + 1).into()
    }
}

#[derive(Debug)]
struct Grid {
    grid: Matrix<Tile>,
    start_tile: Tile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
    position: Vec2<usize>,
    tile: char,
}

fn parse(input: &[String]) -> Grid {
    let mut grid = vec![];
    for (y, row) in input.iter().enumerate() {
        let mut row_buffer = vec![];
        for (x, ch) in row.chars().enumerate() {
            let tile = Tile {
                position: Vec2::new(x, y),
                tile: ch,
            };
            row_buffer.push(tile);
        }
        grid.push(row_buffer);
    }
    let start_tile = grid
        .iter()
        .flatten()
        .find(|tile| tile.tile == 'S')
        .unwrap()
        .clone();
    Grid {
        grid: Matrix::<Tile>::from(grid),
        start_tile,
    }
}

impl Grid {
    fn walk(&self) -> Vec<Vec2<usize>> {
        let mut visited = vec![self.start_tile.position];
        let mut current_tile = self.start_tile.clone();
        loop {
            let mut next_tile: Option<Tile> = None;
            for direction in Cardinal::all_clockwise() {
                let new_pos = direction.advance(Vec2::<isize>::from(current_tile.position));
                if let Some(adjacent) = self.grid.get(new_pos) {
                    if !visited.contains(&adjacent.position)
                        && current_tile.is_connected_to(adjacent, direction)
                    {
                        next_tile = Some(adjacent.clone());
                        break;
                    }
                }
            }
            match next_tile {
                Some(tile) => {
                    current_tile = tile;
                }
                None => break,
            }
            if current_tile == self.start_tile && !visited.is_empty() {
                break;
            }
            visited.push(current_tile.position);
        }
        visited
    }
}

impl Tile {
    fn is_connected_to(&self, other: &Tile, direction: Cardinal) -> bool {
        if other.tile == '.' {
            return false;
        }
        let directions = self.get_connections();
        let other_directions = other.get_connections();
        directions.contains(&direction) && other_directions.contains(&direction.opposite())
    }

    fn get_connections(&self) -> Vec<Cardinal> {
        match self.tile {
            '|' => vec![Cardinal::South, Cardinal::North],
            '-' => vec![Cardinal::East, Cardinal::West],
            'L' => vec![Cardinal::North, Cardinal::East],
            'J' => vec![Cardinal::North, Cardinal::West],
            '7' => vec![Cardinal::South, Cardinal::West],
            'F' => vec![Cardinal::South, Cardinal::East],
            'S' => vec![
                Cardinal::North,
                Cardinal::South,
                Cardinal::East,
                Cardinal::West,
            ],
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day10;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_10_a_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day10.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(4), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_10_b_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day10.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(10), answer);
    }
}
