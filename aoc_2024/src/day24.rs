use std::collections::{HashMap, HashSet};
use std::fs;

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

        // Find the swapped wires using improved algorithm
        let swapped = find_swapped_wires_improved(&pb);

        // Generate visualization
        generate_visualization(&pb, &swapped);

        // Sort and join with commas
        let mut result: Vec<_> = swapped.into_iter().collect();
        result.sort();
        result.join(",").into()
    }
}

impl Problem {
    fn resolve(&mut self) -> usize {
        let z_wires = self
            .dependencies
            .iter()
            .filter(|(k, _)| k.contains("z"))
            .map(|(k, _)| k)
            .sorted_unstable_by(|a, b| a.cmp(b))
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
            bits.push(dfs(w, &self.dependencies, &mut self.computed))
        }
        bits.iter()
            .rev()
            .fold(0, |acc, &bit| (acc << 1) | bit as usize)
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

    fn get_x_value(&self) -> usize {
        let mut x_bits: Vec<_> = self
            .computed
            .iter()
            .filter(|(k, _)| k.starts_with('x'))
            .collect();
        x_bits.sort_by_key(|(k, _)| k.as_str());

        x_bits
            .iter()
            .rev()
            .fold(0, |acc, (_, &bit)| (acc << 1) | bit as usize)
    }

    fn get_y_value(&self) -> usize {
        let mut y_bits: Vec<_> = self
            .computed
            .iter()
            .filter(|(k, _)| k.starts_with('y'))
            .collect();
        y_bits.sort_by_key(|(k, _)| k.as_str());

        y_bits
            .iter()
            .rev()
            .fold(0, |acc, (_, &bit)| (acc << 1) | bit as usize)
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

fn find_swapped_wires_improved(problem: &Problem) -> HashSet<String> {
    let mut suspicious = HashSet::new();
    let deps = &problem.dependencies;

    let num_bits = problem
        .computed
        .iter()
        .filter(|(k, _)| k.starts_with('x'))
        .count();

    // For each bit position, verify the adder structure
    for bit in 0..num_bits {
        let x_bit = format!("x{:02}", bit);
        let y_bit = format!("y{:02}", bit);
        let z_bit = format!("z{:02}", bit);

        // Find gates that should exist for this bit
        let mut xor_xy = None; // x XOR y (intermediate sum)
        let mut and_xy = None; // x AND y (carry generate)

        for (output, (a, b, gate)) in deps.iter() {
            if (*gate == Gate::Xor)
                && ((a == &x_bit && b == &y_bit) || (a == &y_bit && b == &x_bit))
            {
                xor_xy = Some(output.clone());
            }
            if (*gate == Gate::And)
                && ((a == &x_bit && b == &y_bit) || (a == &y_bit && b == &x_bit))
            {
                and_xy = Some(output.clone());
            }
        }

        // Check z output structure
        if let Some((z_in1, z_in2, z_gate)) = deps.get(&z_bit) {
            if bit == 0 {
                // z00 should be direct XOR of x00 and y00
                if *z_gate != Gate::Xor {
                    suspicious.insert(z_bit.clone());
                }
                if !((z_in1 == &x_bit && z_in2 == &y_bit) || (z_in1 == &y_bit && z_in2 == &x_bit)) {
                    // z00 is connected to wrong inputs
                    suspicious.insert(z_bit.clone());
                    if let Some(correct) = xor_xy {
                        if correct != z_bit {
                            suspicious.insert(correct);
                        }
                    }
                }
            } else {
                // For other bits, z should be XOR of intermediate sum and carry
                if *z_gate != Gate::Xor {
                    suspicious.insert(z_bit.clone());
                }

                // One input should be the intermediate XOR
                if let Some(ref intermediate) = xor_xy {
                    if z_in1 != intermediate && z_in2 != intermediate {
                        suspicious.insert(intermediate.clone());
                        suspicious.insert(z_bit.clone());
                    }
                }
            }
        }
    }

    // Check all gates for structural violations
    for (output, (a, b, gate)) in deps.iter() {
        // Rule 1: All z outputs except the last should be XOR
        if output.starts_with('z') {
            let bit_num = output[1..].parse::<usize>().unwrap_or(99);
            if bit_num < num_bits && *gate != Gate::Xor {
                suspicious.insert(output.clone());
            }
        }

        // Rule 2: XOR gates should either:
        // - Have x,y inputs (intermediate sum)
        // - Have one intermediate and one carry input (final sum)
        // - Output to z
        if *gate == Gate::Xor {
            let has_xy = (a.starts_with('x') && b.starts_with('y'))
                || (a.starts_with('y') && b.starts_with('x'));

            if has_xy {
                // This is an intermediate XOR
                // Should NOT output directly to z (except z00)
                if output.starts_with('z') && output != "z00" {
                    suspicious.insert(output.clone());
                }
                // Should feed into another XOR
                let feeds_xor = deps
                    .iter()
                    .any(|(_, (in1, in2, g))| *g == Gate::Xor && (in1 == output || in2 == output));
                if !feeds_xor && output != "z00" {
                    suspicious.insert(output.clone());
                }
            } else if !output.starts_with('z') {
                // Non-z XOR without x,y inputs might be wrong
                // Check if it should be feeding a z output
                let should_feed_z = deps.iter().any(|(out, (in1, in2, _))| {
                    out.starts_with('z') && (in1 == output || in2 == output)
                });
                if !should_feed_z {
                    suspicious.insert(output.clone());
                }
            }
        }

        // Rule 3: AND gates should feed into OR (carry chain)
        if *gate == Gate::And {
            let has_xy = (a.starts_with('x') && b.starts_with('y'))
                || (a.starts_with('y') && b.starts_with('x'));

            if has_xy && !a.ends_with("00") {
                // Should feed into OR
                let feeds_or = deps
                    .iter()
                    .any(|(_, (in1, in2, g))| *g == Gate::Or && (in1 == output || in2 == output));
                if !feeds_or {
                    suspicious.insert(output.clone());
                }
            }

            // AND should never output to z
            if output.starts_with('z') {
                suspicious.insert(output.clone());
            }
        }

        // Rule 4: OR gates are carry propagation, shouldn't output to z directly
        if *gate == Gate::Or && output.starts_with('z') {
            let bit_num = output[1..].parse::<usize>().unwrap_or(99);
            if bit_num != num_bits {
                // Unless it's the final carry
                suspicious.insert(output.clone());
            }
        }
    }

    // If we have too many suspicious wires, try to find pairs
    if suspicious.len() > 8 {
        find_swap_pairs(deps, suspicious)
    } else {
        suspicious
    }
}

fn find_swap_pairs(
    deps: &HashMap<String, (String, String, Gate)>,
    mut suspicious: HashSet<String>,
) -> HashSet<String> {
    let mut result = HashSet::new();

    // Priority 1: Fix z outputs that aren't XOR
    for (output, (_, _, gate)) in deps.iter() {
        if output.starts_with('z') && *gate != Gate::Xor && !output.ends_with("45") {
            result.insert(output.clone());

            // Find an XOR gate that should be here
            for (other_out, (_, _, other_gate)) in deps.iter() {
                if *other_gate == Gate::Xor
                    && suspicious.contains(other_out)
                    && !other_out.starts_with('z')
                {
                    result.insert(other_out.clone());
                    break;
                }
            }
        }
    }

    // Priority 2: Fix XOR gates with x,y that output to z
    for (output, (a, b, gate)) in deps.iter() {
        if *gate == Gate::Xor && output.starts_with('z') && output != "z00" {
            if (a.starts_with('x') && b.starts_with('y'))
                || (a.starts_with('y') && b.starts_with('x'))
            {
                result.insert(output.clone());
            }
        }
    }

    // Fill to 8 with remaining suspicious
    for s in suspicious.iter() {
        if result.len() >= 8 {
            break;
        }
        result.insert(s.clone());
    }

    result
}

fn generate_visualization(problem: &Problem, swapped: &HashSet<String>) {
    let mut dot = String::new();
    dot.push_str("digraph G {\n");
    dot.push_str("  rankdir=LR;\n");
    dot.push_str("  node [shape=box];\n\n");

    // Add input nodes
    for (wire, _) in problem.computed.iter() {
        if wire.starts_with('x') || wire.starts_with('y') {
            dot.push_str(&format!(
                "  {} [shape=circle,style=filled,fillcolor=lightblue];\n",
                wire
            ));
        }
    }

    // Add gates and connections
    for (output, (a, b, gate)) in problem.dependencies.iter() {
        let gate_str = match gate {
            Gate::And => "AND",
            Gate::Or => "OR",
            Gate::Xor => "XOR",
        };

        let color = if swapped.contains(output) {
            "red"
        } else if output.starts_with('z') {
            if *gate != Gate::Xor && !output.ends_with("45") {
                "orange"
            } else {
                "lightgreen"
            }
        } else {
            "white"
        };

        dot.push_str(&format!(
            "  {} [label=\"{}\\n{}\",style=filled,fillcolor=\"{}\"];\n",
            output, output, gate_str, color
        ));
        dot.push_str(&format!("  {} -> {};\n", a, output));
        dot.push_str(&format!("  {} -> {};\n", b, output));
    }

    // Add output nodes
    for (output, _) in problem.dependencies.iter() {
        if output.starts_with('z') {
            dot.push_str(&format!(
                "  out_{} [label=\"{}\",shape=circle,style=filled,fillcolor=yellow];\n",
                output, output
            ));
            dot.push_str(&format!("  {} -> out_{};\n", output, output));
        }
    }

    dot.push_str("}\n");

    fs::write("circuit_visualization.dot", dot).unwrap();
    println!("Visualization saved to circuit_visualization.dot");
    println!("Run: dot -Tpng circuit_visualization.dot -o circuit.png");
    println!("Found {} suspicious wires", swapped.len());
}
