use std::collections::HashSet;

use polynomial::Polynomial;

use aoc_lib::{
    answer::Answer,
    directions::{Cardinal, Direction},
    matrix::Matrix,
    solution::Solution,
    vec2::Vec2,
};

pub struct Day21;

const SIZE: usize = 65;

impl Solution for Day21 {
    fn part_a(&self, input: &[String]) -> Answer {
        let parsed = parse(input);
        let mut queue = HashSet::new();
        queue.insert(parsed.starting_pos);
        for _ in 0..64 {
            let mut next = HashSet::new();
            for pos in queue.iter() {
                for t in next_tiles(&parsed.map, *pos) {
                    next.insert(t);
                }
            }
            queue = next;
        }
        queue.len().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let parsed = parse(input);
        let mut queue = HashSet::new();
        queue.insert(Vec2::<isize>::from(parsed.starting_pos));

        let map = &parsed.map;
        let mut points = vec![];
        let mut steps = 0;
        for run in 0..3 {
            while steps < (65 + run * SIZE) {
                let mut next = HashSet::new();
                for pos in queue.iter() {
                    for t in next_tiles_scaled(map, *pos) {
                        next.insert(t);
                    }
                }
                queue = next;
                steps += 1;
            }
            points.push(Vec2::new(steps, queue.len()));
        }
        let x = points.iter().map(|p| p.x as f64).collect::<Vec<_>>();
        let y = points.iter().map(|p| p.y as f64).collect::<Vec<_>>();
        let pol = Polynomial::lagrange(&x, &y).unwrap();
        (pol.eval(26501365.0).round() as u64).into()
    }
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

fn next_tiles(map: &Matrix<char>, pos: Vec2<usize>) -> Vec<Vec2<usize>> {
    let mut possible = vec![];
    for direction in Cardinal::all_clockwise() {
        let next_pos = Vec2::<isize>::from(pos) + direction.to_offset();
        let next_tile = map.get(&next_pos);
        if let Some(next_tile) = next_tile {
            let next_pos = Vec2::<usize>::try_from(&next_pos).unwrap();
            if *next_tile != '#' {
                possible.push(next_pos);
            }
        }
    }
    possible
}

fn next_tiles_scaled(map: &Matrix<char>, pos: Vec2<isize>) -> Vec<Vec2<isize>> {
    let mut possible = vec![];
    for direction in Cardinal::all_clockwise() {
        let next_pos = pos + direction.to_offset();
        // scaled is to virtually check on the real map
        let scaled = scale_pos(next_pos);
        let next_tile = map[scaled];
        if next_tile != '#' {
            possible.push(next_pos);
        }
    }
    possible
}

fn scale_pos(pos: Vec2<isize>) -> Vec2<usize> {
    let mut mapped = pos;
    // only want
    mapped = Vec2::new(
        (SIZE as isize + mapped.x % SIZE as isize) % SIZE as isize,
        (SIZE as isize + mapped.y % SIZE as isize) % SIZE as isize,
    );
    Vec2::<usize>::try_from(&mapped).unwrap()
}

#[cfg(test)]
mod test {
    use aoc_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day21;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_21_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day21.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(42), answer);
    }
}
