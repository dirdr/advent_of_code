use std::collections::HashMap;

use petgraph::{Graph, Undirected};
use rustworkx_core::connectivity::stoer_wagner_min_cut;

use aoc_lib::{answer::Answer, solution::Solution};

pub struct Day25;

impl Solution for Day25 {
    fn part_a(&self, input: &[String]) -> Answer {
        let graph = parse(input);
        let total = graph.node_count();
        let (len, side) = stoer_wagner_min_cut(&graph, |_| Ok::<i32, ()>(1))
            .unwrap()
            .unwrap();
        assert_eq!(3, len);
        // multiply the two parts together
        ((total - side.len()) * side.len()).into()
    }

    fn part_b(&self, _: &[String]) -> Answer {
        Answer::Unimplemented
    }
}

fn parse(input: &[String]) -> Graph<String, (), Undirected> {
    let mut graph = Graph::new_undirected();
    let mut nodes: HashMap<String, _> = HashMap::new();

    for line in input {
        let (left, right) = line.split_once(": ").unwrap();
        let right_nodes = right.split(' ').collect::<Vec<_>>();

        let left_id = *nodes
            .entry(left.to_string())
            .or_insert_with(|| graph.add_node(left.to_string()));

        for n in &right_nodes {
            let n_id = *nodes
                .entry(n.to_string())
                .or_insert_with(|| graph.add_node(n.to_string()));
            graph.add_edge(left_id, n_id, ());
        }
    }
    graph
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use crate::day25::Day25;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_25_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day25.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(54), answer);
    }
}
