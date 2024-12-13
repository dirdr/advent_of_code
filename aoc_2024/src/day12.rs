use std::collections::{HashSet, VecDeque};

use aoc_lib::{
    answer::Answer,
    directions::{Advance, Cardinal, Direction},
    matrix::Matrix,
    solution::Solution,
    vec2::Vec2,
};

pub struct Day12;

impl Solution for Day12 {
    fn part_a(&self, input: &[String]) -> Answer {
        let mut map = Map::from_input(input);
        map.fence_cost(false).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let mut map = Map::from_input(input);
        map.fence_cost(true).into()
    }
}

struct Map {
    map: Matrix<char>,
    regions: Option<Vec<Region>>,
}

struct Region {
    area: HashSet<Vec2<usize>>,
    perimeter: usize,
    corners: usize,
}

impl Region {
    fn new() -> Self {
        Self {
            area: HashSet::new(),
            perimeter: 0,
            corners: 0,
        }
    }
}

impl Map {
    fn from_input(input: &[String]) -> Self {
        Self {
            map: Matrix::from_chars(input),
            regions: None,
        }
    }

    fn compute_regions(&mut self) {
        let mut all = vec![];
        let mut visited = HashSet::new();
        for y in 0..self.map.rows {
            for x in 0..self.map.cols {
                let pos = Vec2::new(x, y);
                if !visited.contains(&pos) {
                    let region = self.flood(pos, &mut visited);
                    all.push(region);
                }
            }
        }
        self.regions = Some(all);
    }

    fn flood(&self, initial: Vec2<usize>, visited: &mut HashSet<Vec2<usize>>) -> Region {
        let mut region = Region::new();
        let mut queue: VecDeque<Vec2<usize>> = VecDeque::new();
        queue.push_front(initial);
        visited.insert(initial);

        while let Some(pos) = queue.pop_front() {
            region.area.insert(pos);

            for direction in Cardinal::all_clockwise() {
                let next: Vec2<isize> = direction.advance(pos.into());

                let Some(&next_plant_type) = self.map.get(&next) else {
                    region.perimeter += 1;
                    continue;
                };

                if next_plant_type != self.map[pos] {
                    region.perimeter += 1;
                    continue;
                }

                if let Ok(n) = Vec2::<usize>::try_from(next) {
                    if !visited.contains(&n) {
                        visited.insert(n);
                        queue.push_back(n);
                    }
                }
            }
        }
        region
    }

    // The corners can be counted only after all the connected components of the graph
    // has been fully computed
    fn count_corners(&mut self) {
        for region in self.regions.as_mut().unwrap() {
            let area = region
                .area
                .iter()
                .map(|&p| Vec2::<isize>::from(p))
                .collect::<HashSet<_>>();
            for tile in &region.area {
                let sized_tile = Vec2::<isize>::from(tile);
                // Check for corners going inside exterior (Concave from the point of view of the
                // tile)
                // Exemple : You don't have front and right, you are at a corner.
                for direction in Cardinal::all_clockwise() {
                    if !area.contains(&direction.advance(sized_tile))
                        && !area.contains(&direction.turn_right().advance(sized_tile))
                    {
                        region.corners += 1;
                    }
                }

                // Check for corners going inside a hole (Convex from the point of view of the tile)
                // A convex corner is when you have both neighbors but missing the diagonal
                // Exemple : You have front and right but don't have front right, you are at a
                // corner
                for direction in Cardinal::all_clockwise() {
                    let diagonal = direction
                        .turn_right()
                        .advance(direction.advance(sized_tile));
                    if area.contains(&direction.advance(sized_tile))
                        && area.contains(&direction.turn_right().advance(sized_tile))
                        && !area.contains(&diagonal)
                    {
                        region.corners += 1;
                    }
                }
            }
        }
    }

    fn fence_cost(&mut self, discount: bool) -> usize {
        self.compute_regions();
        if discount {
            self.count_corners();
        }
        let mut total = 0;
        for region in self.regions.as_ref().unwrap() {
            let mul = if discount {
                region.corners
            } else {
                region.perimeter
            };
            total += region.area.len() * mul;
        }
        total
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day12;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_12_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day12.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(1930), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_12_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day12.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(1206), answer);
    }
}
