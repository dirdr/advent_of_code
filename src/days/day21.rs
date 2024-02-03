use std::collections::{HashSet, VecDeque};

use crate::helper_lib::{
    answer::Answer, directions::Direction, matrix::Matrix, solution::Solution, vec2::Vec2,
};

pub struct Day21;

impl Solution for Day21 {
    fn part_a(&self, input: &[String]) -> Answer {
        let parsed = parse(input);
        let bfs = solve_bfs(&parsed);
        bfs.into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        todo!()
    }
}

// horrible performance, i think because of ciruclar path finding with large number of step
// bfs impl better here
#[allow(dead_code)]
fn solve_dfs(parsed: &Parsed) -> usize {
    let mut visited = HashSet::new();
    fn dfs(
        map: &Matrix<char>,
        pos: Vec2<usize>,
        step_left: usize,
        visited: &mut HashSet<Vec2<usize>>,
    ) {
        if step_left == 64 {
            visited.insert(pos);
            return;
        }

        for t in next_tiles(map, pos) {
            dfs(map, t, step_left + 1, visited);
        }
    }
    dfs(&parsed.map, parsed.starting_pos, 0, &mut visited);
    visited.len()
}

fn solve_bfs(parsed: &Parsed) -> usize {
    let map = &parsed.map;
    let mut queue = HashSet::new();
    queue.insert(parsed.starting_pos);
    for _ in 0..64 {
        let mut next = HashSet::new();
        for pos in queue.iter() {
            for t in next_tiles(map, *pos) {
                next.insert(t);
            }
        }
        std::mem::swap(&mut queue, &mut next);
    }
    queue.len()
}

fn next_tiles(map: &Matrix<char>, pos: Vec2<usize>) -> Vec<Vec2<usize>> {
    let mut possible = vec![];
    let directions = Direction::all();
    for direction in directions {
        let next_pos = Vec2::<isize>::from(pos) + direction.to_offset();
        let next_tile = map.get(next_pos);
        if let Some(next_tile) = next_tile {
            let next_pos = Vec2::<usize>::try_from(next_pos).unwrap();
            if *next_tile != '#' {
                possible.push(next_pos);
            }
        }
    }
    possible
}

struct Parsed {
    map: Matrix<char>,
    starting_pos: Vec2<usize>,
}

fn parse(input: &[String]) -> Parsed {
    let map = Matrix::from_chars(input);
    let starting_pos = map.find('S').unwrap();
    Parsed { map, starting_pos }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day21;

    #[test]
    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_21_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day21.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(42), answer);
    }

    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_21_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day21.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(16), answer);
    }
}
