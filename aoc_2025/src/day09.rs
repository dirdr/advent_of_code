use aoc_lib::{answer::Answer, solution::Solution};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day9;

impl Solution for Day9 {
    fn part_a(&self, input: &[String]) -> Answer {
        Floor::from_input(input).part_a().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        Floor::from_input(input).part_b().into()
    }
}

struct Floor {
    original: Vec<(i64, i64)>,
    compressed: Vec<(i64, i64)>,
}

impl Floor {
    fn from_input(input: &[String]) -> Self {
        let original: Vec<(i64, i64)> = input
            .iter()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();

        let compressed = Self::compress_coordinates(&original);

        Self {
            original,
            compressed,
        }
    }

    fn compress_coordinates(coords: &[(i64, i64)]) -> Vec<(i64, i64)> {
        let mut x_vals: Vec<i64> = coords.iter().map(|(x, _)| *x).collect();
        let mut y_vals: Vec<i64> = coords.iter().map(|(_, y)| *y).collect();

        x_vals.sort_unstable();
        x_vals.dedup();
        y_vals.sort_unstable();
        y_vals.dedup();

        let x_map: HashMap<i64, i64> = x_vals
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i as i64 * 2))
            .collect();

        let y_map: HashMap<i64, i64> = y_vals
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i as i64 * 2))
            .collect();

        coords.iter().map(|(x, y)| (x_map[x], y_map[y])).collect()
    }

    fn part_a(&self) -> i64 {
        (0..self.original.len())
            .flat_map(|i| ((i + 1)..self.original.len()).map(move |j| (i, j)))
            .map(|(i, j)| Self::rectangle_area(self.original[i], self.original[j]))
            .max()
            .unwrap_or(0)
    }

    fn part_b(&self) -> i64 {
        let border = self.get_border(&self.compressed);

        let min_x = *self.compressed.iter().map(|(x, _)| x).min().unwrap();
        let max_x = *self.compressed.iter().map(|(x, _)| x).max().unwrap();
        let min_y = *self.compressed.iter().map(|(_, y)| y).min().unwrap();
        let max_y = *self.compressed.iter().map(|(_, y)| y).max().unwrap();

        let outside = self.fill_outside(&border, min_x - 1, max_x + 1, min_y - 1, max_y + 1);

        let mut pairs: Vec<(usize, usize)> = (0..self.original.len())
            .flat_map(|i| ((i + 1)..self.original.len()).map(move |j| (i, j)))
            .collect();

        pairs.sort_unstable_by_key(|(i, j)| {
            std::cmp::Reverse(Self::rectangle_area(self.original[*i], self.original[*j]))
        });

        for (i, j) in pairs {
            let vertices = Self::rectangle_vertices(self.compressed[i], self.compressed[j]);

            if vertices.iter().any(|v| outside.contains(v)) {
                continue;
            }

            if self
                .rectangle_border(self.compressed[i], self.compressed[j])
                .is_disjoint(&outside)
            {
                return Self::rectangle_area(self.original[i], self.original[j]);
            }
        }

        0
    }

    fn rectangle_area(p1: (i64, i64), p2: (i64, i64)) -> i64 {
        ((p1.0 - p2.0).abs() + 1) * ((p1.1 - p2.1).abs() + 1)
    }

    fn rectangle_vertices(p1: (i64, i64), p2: (i64, i64)) -> Vec<(i64, i64)> {
        vec![p1, (p1.0, p2.1), p2, (p2.0, p1.1)]
    }

    fn get_line(p1: (i64, i64), p2: (i64, i64)) -> HashSet<(i64, i64)> {
        let mut line = HashSet::new();

        if p1.0 == p2.0 {
            let (min_y, max_y) = (p1.1.min(p2.1), p1.1.max(p2.1));
            for y in min_y..=max_y {
                line.insert((p1.0, y));
            }
        } else if p1.1 == p2.1 {
            let (min_x, max_x) = (p1.0.min(p2.0), p1.0.max(p2.0));
            for x in min_x..=max_x {
                line.insert((x, p1.1));
            }
        }

        line
    }

    fn get_border(&self, coords: &[(i64, i64)]) -> HashSet<(i64, i64)> {
        (0..coords.len())
            .flat_map(|i| Self::get_line(coords[i], coords[(i + 1) % coords.len()]))
            .collect()
    }

    fn rectangle_border(&self, p1: (i64, i64), p2: (i64, i64)) -> HashSet<(i64, i64)> {
        self.get_border(&Self::rectangle_vertices(p1, p2))
    }

    fn fill_outside(
        &self,
        border: &HashSet<(i64, i64)>,
        min_x: i64,
        max_x: i64,
        min_y: i64,
        max_y: i64,
    ) -> HashSet<(i64, i64)> {
        let mut outside = HashSet::new();
        let mut queue = VecDeque::from([(min_x, min_y)]);
        outside.insert((min_x, min_y));

        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (nx, ny) = (x + dx, y + dy);

                if nx >= min_x
                    && nx <= max_x
                    && ny >= min_y
                    && ny <= max_y
                    && !border.contains(&(nx, ny))
                    && outside.insert((nx, ny))
                {
                    queue.push_back((nx, ny));
                }
            }
        }

        outside
    }
}

#[cfg(test)]
mod test {
    use super::Day9;
    use aoc_lib::{answer::Answer, input, solution::Solution};

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_09_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        assert_eq!(<i64 as Into<Answer>>::into(50), Day9.part_a(&input));
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_09_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        assert_eq!(<i64 as Into<Answer>>::into(24), Day9.part_b(&input));
    }
}
