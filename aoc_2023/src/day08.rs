use std::collections::HashMap;

use aoc_lib::{answer::Answer, maths, solution::Solution};

pub struct Day8;

impl Solution for Day8 {
    fn part_a(&self, input: &[String]) -> Answer {
        let parsed = parse(input);
        let mut current_node = "AAA";
        for (count, direction) in parsed.instructions.iter().cycle().enumerate() {
            if current_node == "ZZZ" {
                return count.into();
            }
            current_node = parsed.get(current_node, direction);
        }
        0.into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let parsed = parse(input);
        let starting_nodes: Vec<&str> = parsed
            .network
            .iter()
            .filter(|(&k, _)| k.ends_with('A'))
            .map(|(&k, _)| k)
            .collect();
        let mut cycles = vec![];
        for node in starting_nodes {
            let mut current_node = node;
            for (cycle_len, direction) in parsed.instructions.iter().cycle().enumerate() {
                if current_node.ends_with('Z') {
                    cycles.push(cycle_len as i64);
                    break;
                }
                current_node = parsed.get(current_node, direction);
            }
        }
        let lcm = cycles.iter().copied().reduce(maths::lcm).unwrap_or(1);
        lcm.into()
    }
}

struct Parsed<'a> {
    instructions: Vec<char>,
    network: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Parsed<'a> {
    fn get(&self, el: &'a str, direction: &char) -> &'a str {
        let entry = self.network.get(el).unwrap();
        match direction {
            'L' => entry.0,
            'R' => entry.1,
            _ => unreachable!(),
        }
    }
}

fn parse(input: &[String]) -> Parsed<'_> {
    let split_index = input
        .iter()
        .enumerate()
        .find(|&(_, line)| line.trim().is_empty())
        .map_or(input.len(), |(index, _)| index);

    let (instructions, nodes) = input.split_at(split_index);
    let instructions = instructions
        .iter()
        .flat_map(|s| s.chars())
        .collect::<Vec<_>>();
    let mut network = HashMap::new();
    for line in nodes.iter().skip(1) {
        let (node, other) = line.split_once('=').unwrap();
        let (left, right) = other.trim().split_once(',').unwrap();
        let left = &left[1..];
        let right = &right[..right.len() - 1];
        network.insert(node.trim(), (left.trim(), right.trim()));
    }
    Parsed {
        instructions,
        network,
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day8;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_08_a_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day8.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(2), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_08_b_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day8.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(6), answer);
    }
}
