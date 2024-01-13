use std::collections::{HashMap, HashSet, VecDeque};

use crate::helper_lib::{
    answer::Answer, directions::Direction, matrix::Matrix, solution::Solution, vec2::Vec2,
};

pub struct Day17;

impl Solution for Day17 {
    fn part_a(&self, input: &[String]) -> Answer {
        let grid = parse(input);
        grid.minimum_path_heat_loss().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        todo!()
    }
}

struct Grid {
    grid: Matrix<usize>,
}

impl Grid {
    fn minimum_path_heat_loss(&self) -> usize {
        fn bfs(grid: &Matrix<usize>) -> usize {
            let mut queue = VecDeque::new();
            let mut seen: HashMap<Node, usize> = HashMap::new();
            let start = Vec2::new(0, 0);
            let end = Vec2::new(grid.cols - 1, grid.rows - 1);

            for direction in [Direction::East, Direction::South] {
                let node = Node::new(start, direction, 3);
                queue.push_back((node, 0));
                seen.insert(node, 0);
            }

            let mut min = usize::MAX;

            while !queue.is_empty() {
                let mut next_queue = VecDeque::new();
                while let Some((node, cost)) = queue.pop_front() {
                    // arrived at the end
                    if node.pos == end && node.turn_counter > 0 {
                        min = min.min(cost);
                        continue;
                    }

                    // explore all (avaibles) neighboors
                    for dir in Grid::avaible_directions(grid, &node) {
                        let next_pos = Vec2::<usize>::try_from(
                            Vec2::<isize>::from(node.pos) + dir.to_offset(),
                        )
                        .unwrap();
                        let counter = if dir == node.facing {
                            node.turn_counter - 1
                        } else {
                            2 // because we just moved so decrement 3 by one
                        };
                        let next_node = Node::new(next_pos, dir, counter);
                        let cost = cost + grid[next_node.pos];
                        if !seen.contains_key(&next_node) || *seen.get(&next_node).unwrap() > cost {
                            next_queue.push_back((next_node, cost));
                            seen.insert(next_node, cost);
                        }
                    }
                }
                std::mem::swap(&mut queue, &mut next_queue);
            }
            min
        }
        bfs(&self.grid)
    }

    fn avaible_directions(grid: &Matrix<usize>, curr: &Node) -> Vec<Direction> {
        // at each state, you can continue straight, go left or right.
        // if the turn distance is 0 you must turn left or right and the straight
        // option will not be avaible
        let mut avaibles = vec![curr.facing.turn_left(), curr.facing.turn_right()];
        if curr.turn_counter > 0 {
            avaibles.push(curr.facing);
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
        assert_eq!(<i32 as Into<Answer>>::into(51), answer);
    }
}
