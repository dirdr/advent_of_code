use crate::helper_lib::{answer::Answer, directions::Direction, solution::Solution};

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
                acc + (p.0 as i32 * path[l].1 as i32 - p.1 as i32 * path[l].0 as i32)
            })
            .abs()
            / 2;
        (area - (len as i32 / 2) + 1).into()
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Tile>>,
    start_tile: Tile,
}

fn parse(input: &[String]) -> Grid {
    let mut grid = vec![];
    for (i, row) in input.iter().enumerate() {
        let mut row_buffer = vec![];
        for (j, col) in row.chars().enumerate() {
            let tile = Tile {
                position: (j, i),
                tile: col,
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
    Grid { grid, start_tile }
}

impl Grid {
    fn walk(&self) -> Vec<(usize, usize)> {
        let mut visited = vec![self.start_tile.position];
        let mut current_tile = self.start_tile.clone();
        loop {
            let mut next_tile: Option<Tile> = None;
            for direction in Direction::all() {
                if let Some(adjacent) = self.try_advance(&current_tile, &direction) {
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

    fn try_advance(&self, tile: &Tile, direction: &Direction) -> Option<&Tile> {
        let (x_offset, y_offset) = direction.to_offset();
        let (x, y) = (tile.position.0, tile.position.1);
        let (new_x, new_y) = (x as isize + x_offset, y as isize + y_offset);
        // check if out of bounds
        if (new_x < 0)
            || (new_x >= self.grid[0].len() as isize)
            || (new_y < 0)
            || (new_y >= self.grid.len() as isize)
        {
            return None;
        }
        Some(&self.grid[new_y as usize][new_x as usize])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
    position: (usize, usize),
    tile: char,
}

impl Tile {
    fn is_connected_to(&self, other: &Tile, direction: Direction) -> bool {
        if other.tile == '.' {
            return false;
        }
        let directions = self.get_connections();
        let other_directions = other.get_connections();
        directions.contains(&direction) && other_directions.contains(&direction.opposite())
    }

    fn get_connections(&self) -> Vec<Direction> {
        match self.tile {
            '|' => vec![Direction::South, Direction::North],
            '-' => vec![Direction::East, Direction::West],
            'L' => vec![Direction::North, Direction::East],
            'J' => vec![Direction::North, Direction::West],
            '7' => vec![Direction::South, Direction::West],
            'F' => vec![Direction::South, Direction::East],
            'S' => vec![
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ],
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day10;

    #[test]
    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_10_a_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day10.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(4), answer);
    }

    #[test]
    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_10_b_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day10.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(10), answer);
    }
}
