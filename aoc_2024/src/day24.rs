use std::collections::{HashMap, HashSet};

use aoc_lib::{answer::Answer, solution::Solution};
use itertools::Itertools;

pub struct Day24;

impl Solution for Day24 {
    fn part_a(&self, input: &[String]) -> Answer {
        let mut pb = Problem::from_input(input);
        pb.resolve().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let pb = Problem::from_input(input);
        pb.analyze().into()
    }
}

impl Problem {
    fn resolve(&mut self) -> usize {
        let z_wires = self
            .dependencies
            .iter()
            .filter(|(k, _)| k.starts_with('z'))
            .map(|(k, _)| k)
            .sorted_unstable()
            .rev()
            .cloned()
            .collect::<Vec<String>>();

        fn dfs(
            wire: &String,
            dependencies: &HashMap<String, (String, String, Gate)>,
            computed: &mut HashMap<String, u8>,
        ) -> u8 {
            if let Some(value) = computed.get(wire) {
                return *value;
            }
            let (a, b, gate) = dependencies.get(wire).unwrap();
            let left = dfs(a, dependencies, computed);
            let right = dfs(b, dependencies, computed);
            let result = gate.apply(left, right);
            computed.insert(wire.clone(), result);
            result
        }

        let mut bits = vec![];
        for w in &z_wires {
            bits.push(dfs(w, &self.dependencies, &mut self.computed));
        }

        bits.iter().fold(0, |acc, &bit| (acc << 1) | bit as usize)
    }

    fn from_input(input: &[String]) -> Self {
        let mut parts = input.splitn(2, |e| e.is_empty());
        let mut dependencies = HashMap::new();
        let mut initial_wires = HashMap::new();
        let mut computed = HashMap::new();

        for line in parts.next().unwrap() {
            let (name, value) = line.split_once(':').unwrap();
            let value = value.trim().parse::<u8>().unwrap();
            let wire = name.to_string();
            initial_wires.insert(name, wire.clone());
            computed.insert(wire, value);
        }

        let create_wire = |name: &str| {
            if let Some(w) = initial_wires.get(name) {
                w.clone()
            } else {
                name.to_string()
            }
        };

        for line in parts.next().unwrap() {
            let fragment = line.split_whitespace().collect::<Vec<_>>();
            let gate_type = Gate::from(fragment[1]);
            dependencies.insert(
                create_wire(fragment[4]),
                (
                    create_wire(fragment[0]),
                    create_wire(fragment[2]),
                    gate_type,
                ),
            );
        }

        Self {
            dependencies,
            computed,
        }
    }

    fn analyze(&self) -> String {
        let mut faulty = HashSet::new();

        let max_z = self
            .dependencies
            .keys()
            .filter(|k| k.starts_with('z'))
            .max()
            .cloned()
            .unwrap();

        for (output, (in1, in2, gate)) in &self.dependencies {
            if output.starts_with("z") && gate != &Gate::Xor && output != &max_z {
                faulty.insert(output.clone());
            }

            if !output.starts_with("z") && !self.has_xy_inputs(in1, in2) && gate == &Gate::Xor {
                faulty.insert(output.clone());
            }

            // If we found a xor gate with input x and y, whose outputting v then we must have an xor gate
            // whose input is v.
            if gate == &Gate::Xor
                && self.has_xy_inputs(in1, in2)
                && !self.is_initial_carry(in1, in2)
            {
                let mut is_valid = false;
                for v in self.dependencies.values() {
                    if v.2 == Gate::Xor && (output == &v.0 || output == &v.1) {
                        is_valid = true;
                    }
                }
                if !is_valid {
                    faulty.insert(output.clone());
                }
            }

            // If we found a xor gate with input x and y, whose outputting v then we must have an xor gate
            // whose input is v.
            if gate == &Gate::And && !self.is_initial_carry(in1, in2) {
                let mut is_valid = false;
                for v in self.dependencies.values() {
                    if v.2 == Gate::Or && (output == &v.0 || output == &v.1) {
                        is_valid = true;
                    }
                }
                if !is_valid {
                    faulty.insert(output.clone());
                }
            }
        }

        faulty.iter().sorted().join(",")
    }

    fn has_xy_inputs(&self, in1: &str, in2: &str) -> bool {
        (in1.starts_with('x') || in1.starts_with('y'))
            && (in2.starts_with('x') || in2.starts_with('y'))
    }

    fn is_initial_carry(&self, in1: &str, in2: &str) -> bool {
        (in1 == "x00" && in2 == "y00") || (in1 == "y00" && in2 == "x00")
    }
}

struct Problem {
    dependencies: HashMap<String, (String, String, Gate)>,
    computed: HashMap<String, u8>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Gate {
    And,
    Or,
    Xor,
}

impl Gate {
    fn apply(&self, a: u8, b: u8) -> u8 {
        match self {
            Gate::And => a & b,
            Gate::Or => a | b,
            Gate::Xor => a ^ b,
        }
    }
}

impl From<&str> for Gate {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "XOR" => Gate::Xor,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day24;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_24_a_test.txt", crate::FILES_PREFIX_TEST)).unwrap();

        let answer = Day24.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(2024), answer);
    }
}
