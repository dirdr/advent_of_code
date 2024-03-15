use std::collections::{HashMap, HashSet};

use crate::helper_lib::{answer::Answer, solution::Solution, vec2::Vec2};
use crate::helper_lib::{directions::Direction, matrix::Matrix};

pub struct Day23;

impl Solution for Day23 {
    fn part_a(&self, input: &[String]) -> Answer {
        solve(input, false).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        solve(input, true).into()
    }
}

fn solve(input: &[String], ignore_slopes: bool) -> usize {
    let map = parse(input);
    let mut graph = construct_graph(&map, ignore_slopes);
    graph.collapse();
    longest_path(
        &graph,
        &map.get_starting_position(),
        &map.get_goal_position(),
    )
}

fn longest_path(graph: &Graph, starting_pos: &Vec2<usize>, goal: &Vec2<usize>) -> usize {
    let mut visited = HashSet::new();
    fn dfs(
        graph: &Graph,
        curr: &Node,
        goal: &Node,
        visited: &mut HashSet<Node>,
        current: usize,
    ) -> usize {
        if curr == goal {
            return current;
        }

        visited.insert(*curr);
        let mut max_path = 0;

        for child in graph.adjacency_list[curr].iter() {
            if !visited.contains(&child.0) {
                let path_len = dfs(graph, &child.0, goal, visited, current + child.1);
                max_path = max_path.max(path_len)
            }
        }
        // to allow futur path to explore by this node
        visited.remove(curr);
        max_path
    }
    dfs(graph, starting_pos, goal, &mut visited, 0)
}

type Node = Vec2<usize>;
type WeightedNextNode = (Vec2<usize>, usize);

#[derive(Debug)]
struct Graph {
    // stored as {position : vec [(position, weight)]}
    adjacency_list: HashMap<Node, HashSet<WeightedNextNode>>,
}

impl Graph {
    // collapse the graph to remove un-necesary channels
    // From :
    // A---B---C
    //   2   3
    // To :
    // A---C
    //   5
    fn collapse(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            for key in self.adjacency_list.keys().copied().collect::<Vec<_>>() {
                // can collapse a node only if it is inside a channel
                let neighbours = self.adjacency_list[&key].clone();
                if neighbours.len() != 2 {
                    continue;
                }
                let (a, b) = (
                    neighbours.iter().nth(0).unwrap(),
                    neighbours.iter().nth(1).unwrap(),
                );
                let cost = a.1 + b.1;

                let a_neighbours = self.adjacency_list.get_mut(&a.0).unwrap();
                a_neighbours.retain(|(pos, _)| *pos != key);
                a_neighbours.insert((b.0, cost));

                let b_neighbours = self.adjacency_list.get_mut(&b.0).unwrap();
                b_neighbours.retain(|(pos, _)| *pos != key);
                b_neighbours.insert((a.0, cost));

                self.adjacency_list.remove(&key);
                changed = true;
            }
        }
    }
}

fn construct_graph(map: &Map, ignore_slopes: bool) -> Graph {
    let mut adjacency_list: HashMap<Node, HashSet<WeightedNextNode>> = HashMap::new();
    for r in 0..map.map.rows {
        for c in 0..map.map.cols {
            let pos = Vec2::new(c, r);
            let tile = map.map[pos];
            if tile != '#' {
                // each step is represented as weight of 1
                let next_tiles = map
                    .avaible_next_pos(&pos, ignore_slopes)
                    .into_iter()
                    .map(|pos| (pos, 1))
                    .collect::<Vec<_>>();
                adjacency_list.entry(pos).or_default().extend(next_tiles)
            }
        }
    }
    Graph { adjacency_list }
}

impl Map {
    fn get_starting_position(&self) -> Vec2<usize> {
        for c in 0..self.map.cols {
            let pos = Vec2::new(c, 0);
            if self.map[pos] == '.' {
                return pos;
            }
        }
        // guarenteed to be in the first row
        unreachable!()
    }

    fn get_goal_position(&self) -> Vec2<usize> {
        for c in 0..self.map.cols {
            let pos = Vec2::new(c, self.map.rows - 1);
            if self.map[pos] == '.' {
                return pos;
            }
        }
        // guarenteed to be in the last row
        unreachable!()
    }

    fn avaible_next_pos(&self, pos: &Vec2<usize>, ignore_slope: bool) -> Vec<Vec2<usize>> {
        let mut directions = vec![];
        if !ignore_slope {
            for (ch, direction) in SLOPES.iter() {
                if self.map[*pos] == *ch {
                    directions.push(*direction);
                }
            }
        }
        let mut possible = vec![];
        // not on a slope
        if directions.is_empty() {
            directions = Direction::all().collect::<Vec<_>>();
        }
        for direction in directions {
            let next_pos = Vec2::<isize>::from(*pos) + direction.to_offset();
            let next_tile = self.map.get(next_pos);
            if let Some(next_tile) = next_tile {
                let next_pos = Vec2::<usize>::try_from(next_pos).unwrap();
                if *next_tile != '#' {
                    possible.push(next_pos);
                }
            }
        }
        possible
    }
}

const SLOPES: [(char, Direction); 4] = [
    ('>', Direction::East),
    ('<', Direction::West),
    ('^', Direction::North),
    ('v', Direction::South),
];

fn parse(input: &[String]) -> Map {
    Map {
        map: Matrix::from_chars(input),
    }
}

struct Map {
    map: Matrix<char>,
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day23;

    #[test]
    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_23_test.txt",
            helper_lib::consts::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day23.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(94), answer);
    }

    #[test]
    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_23_test.txt",
            helper_lib::consts::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day23.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(154), answer);
    }
}
