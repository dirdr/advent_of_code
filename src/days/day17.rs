use std::collections::{HashMap, VecDeque};

use crate::helper_lib::{
    answer::Answer, directions::Direction, matrix::Matrix, solution::Solution, vec2::Vec2,
};

pub struct Day17;

impl Solution for Day17 {
    fn part_a(&self, input: &[String]) -> Answer {
        let grid = parse(input);
        grid.minimum_path_heat_loss(1, 3).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let grid = parse(input);
        grid.minimum_path_heat_loss(4, 10).into()
    }
}

struct Grid {
    grid: Matrix<usize>,
}

impl Grid {
    fn minimum_path_heat_loss(&self, min_dist: usize, max_dist: usize) -> usize {
        let mut queue = VecDeque::new();
        let mut seen: HashMap<Node, usize> = HashMap::new();
        let start = Vec2::new(0, 0);
        let end = Vec2::new(self.grid.cols - 1, self.grid.rows - 1);

        for direction in [Direction::East, Direction::South] {
            let node = Node::new(start, direction, 1);
            queue.push_back((node, 0));
            seen.insert(node, 0);
        }

        let mut min = usize::MAX;

        while let Some((node, cost)) = queue.pop_front() {
            // arrived at the end
            if node.pos == end && node.turn_counter >= min_dist {
                min = min.min(cost);
                continue;
            }

            // explore all (avaibles) neighboors
            for dir in Grid::avaible_directions(&self.grid, &node, min_dist, max_dist) {
                let next_pos =
                    Vec2::<usize>::try_from(Vec2::<isize>::from(node.pos) + dir.to_offset())
                        .unwrap();
                let counter = if dir == node.facing {
                    node.turn_counter + 1
                } else {
                    1
                };
                let next_node = Node::new(next_pos, dir, counter);
                let cost = cost + self.grid[next_node.pos];
                if !seen.contains_key(&next_node) || *seen.get(&next_node).unwrap() > cost {
                    queue.push_back((next_node, cost));
                    seen.insert(next_node, cost);
                }
            }
        }
        min
    }

    fn avaible_directions(
        grid: &Matrix<usize>,
        curr: &Node,
        min_dist: usize,
        max_dist: usize,
    ) -> Vec<Direction> {
        // at each state, you can continue straight, go left or right.
        // if the turn distance is 0 you must turn left or right and the straight
        // option will not be avaible
        let mut avaibles = vec![];
        if curr.turn_counter < max_dist {
            avaibles.push(curr.facing);
        }
        if curr.turn_counter >= min_dist {
            avaibles.push(curr.facing.turn_right());
            avaibles.push(curr.facing.turn_left());
        }
        avaibles
            .into_iter()
            .filter(|d| grid.contains(Vec2::<isize>::from(curr.pos) + d.to_offset()))
            .collect()
    }
}

fn parse(input: &[String]) -> Grid {
    Grid {
        grid: Matrix::from_chars(input)
            .map_to(|c: char| c.to_digit(10).unwrap_or_default() as usize),
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Node {
    pos: Vec2<usize>,
    facing: Direction,
    turn_counter: usize,
}

impl Node {
    fn new(pos: Vec2<usize>, facing: Direction, turn_counter: usize) -> Self {
        Self {
            pos,
            facing,
            turn_counter,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day17;

    #[test]
    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_17_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day17.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(102), answer);
    }

    #[test]
    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_17_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day17.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(94), answer);
    }
}
