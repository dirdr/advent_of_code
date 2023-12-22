use std::collections::HashMap;

use crate::helper_lib::{answer::Answer, lcm, solution::Solution};

pub struct Day8;

struct Parsed<'a> {
    instructions: Vec<char>,
    network: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Parsed<'a> {
    fn get(&self, el: &'a str, direction: &char) -> &'a str {
        let entry = self.network.get(el).unwrap();
        match direction {
            'L' => &entry.0,
            'R' => &entry.1,
            _ => unreachable!(),
        }
    }
}

fn parse(input: &[String]) -> Parsed {
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
        let (node, other) = line.split_once("=").unwrap();
        let (left, right) = other.trim().split_once(",").unwrap();
        let left = &left[1..];
        let right = &right[..right.len() - 1];
        network.insert(node.trim(), (left.trim(), right.trim()));
    }
    Parsed {
        instructions,
        network,
    }
}

impl Solution for Day8 {
    fn part_a(&self, input: &[String]) -> Answer {
        let parsed = parse(input);
        let mut count = 0;
        let mut current_node = "AAA";
        for direction in parsed.instructions.iter().cycle() {
            if current_node == "ZZZ" {
                return count.into();
            }
            current_node = parsed.get(current_node, &direction);
            count += 1;
        }
        0.into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let parsed = parse(input);
        let starting_nodes: Vec<&str> = parsed
            .network
            .iter()
            .filter(|(&k, _)| k.ends_with("A"))
            .map(|(&k, _)| k)
            .collect();
        let mut cycles = vec![];
        for node in starting_nodes {
            let mut cycle_len = 0;
            let mut current_node = node;
            for direction in parsed.instructions.iter().cycle() {
                if current_node.ends_with('Z') {
                    cycles.push(cycle_len);
                    break;
                }
                current_node = parsed.get(current_node, &direction);
                cycle_len += 1;
            }
        }
        let lcm = cycles.iter().copied().reduce(|a, b| lcm(a, b)).unwrap_or(1);
        lcm.into()
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day8;

    #[test]
    pub fn test_a() {
        let input =
            input::read_file(&format!("{}day_8_a_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day8.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(2i32), answer);
    }

    #[test]
    pub fn test_b() {
        let input =
            input::read_file(&format!("{}day_8_b_test.txt", helper_lib::FILES_PREFIX)).unwrap();
        let answer = Day8.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(6i32), answer);
    }
}
