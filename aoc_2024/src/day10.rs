use std::collections::HashSet;

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
        let map = Map::from_input(input);
        map.trailheads_metric(MetricType::Score).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let map = Map::from_input(input);
        map.trailheads_metric(MetricType::Rating).into()
    }
}

enum MetricType {
    Score,
    Rating,
}

struct Map {
    map: Matrix<u8>,
}

impl Map {
    fn from_input(input: &[String]) -> Self {
        let map = Matrix::from_chars(input).map_to(|c| c.to_digit(10).unwrap() as u8);
        Self { map }
    }

    fn trailheads_metric(&self, metric: MetricType) -> usize {
        let mut total = 0;
        for y in 0..self.map.rows {
            for x in 0..self.map.cols {
                let pos = Vec2::new(x, y);
                if self.map[pos] != 0 {
                    continue;
                }
                total += match metric {
                    MetricType::Score => self.trailhead_score(pos, &mut HashSet::new()),
                    MetricType::Rating => self.trailhead_rating(pos),
                }
            }
        }
        total
    }

    /// The trailhead score is the number of reachable `9` from the given trailhead.
    /// We need to maintain a set of visited position to avoid exploring the same '9' multiple time
    /// from different hiking trail.
    fn trailhead_score(&self, current: Vec2<usize>, visited: &mut HashSet<Vec2<usize>>) -> usize {
        if !visited.insert(current) {
            return 0;
        }

        let height = self.map[current];
        if height == 9 {
            return 1;
        }

        let mut count = 0;
        for d in Cardinal::all_clockwise() {
            let current_signed: Vec2<isize> = current.into();
            let next = d.advance(current_signed);

            let Some(next_height) = self.map.get(&next) else {
                continue;
            };

            if next_height.saturating_sub(height) != 1 {
                continue;
            }

            if let Ok(safe_next) = Vec2::<usize>::try_from(&next) {
                count += self.trailhead_score(safe_next, visited);
            }
        }
        count
    }

    /// The trailhead rating is the number of distinct hiking trails from the given trailhead.
    /// This time, we don't maintain a list of visited node because from a given pos in the graph,
    /// we want to count different path that lead to the same end because those are different
    /// hinking trails.
    fn trailhead_rating(&self, current: Vec2<usize>) -> usize {
        let height = self.map[current];
        if height == 9 {
            return 1;
        }

        let mut count = 0;
        for d in Cardinal::all_clockwise() {
            let current_signed: Vec2<isize> = current.into();
            let next = d.advance(current_signed);

            let Some(next_height) = self.map.get(&next) else {
                continue;
            };

            if next_height.saturating_sub(height) != 1 {
                continue;
            }

            if let Ok(safe_next) = Vec2::<usize>::try_from(&next) {
                count += self.trailhead_rating(safe_next);
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day10;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_10_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day10.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(36), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_10_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day10.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(81), answer);
    }
}
