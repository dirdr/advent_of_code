use std::collections::{HashMap, HashSet};

use aoc_lib::{answer::Answer, solution::Solution};

pub struct Day11;

impl Solution for Day11 {
    fn part_a(&self, input: &[String]) -> Answer {
        Graph::from_input(input)
            .count_valid_paths("you", false)
            .into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        Graph::from_input(input)
            .count_valid_paths("svr", true)
            .into()
    }
}

struct Graph<'a> {
    adj: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Graph<'a> {
    fn from_input(input: &'a [String]) -> Self {
        let mut adj = HashMap::new();
        for line in input {
            let (src, dests) = line.split_once(':').unwrap();
            let dests = dests.split_whitespace().collect::<Vec<_>>();
            adj.insert(src, dests);
        }
        Self { adj }
    }

    fn count_valid_paths(&self, start: &str, constrained: bool) -> usize {
        fn dfs<'a>(
            adj: &HashMap<&'a str, Vec<&'a str>>,
            key: &'a str,
            visited: &mut HashSet<&'a str>,
            memo: &mut HashMap<(&'a str, [bool; 2]), usize>,
            seen @ [fft, dac]: [bool; 2],
        ) -> usize {
            let entry = (key, seen);
            if let Some(&cached) = memo.get(&entry) {
                return cached;
            }

            if key == "out" && fft && dac {
                return 1;
            }

            let mut count = 0;
            if let Some(dests) = adj.get(&key) {
                for &dest in dests {
                    if visited.contains(dest) {
                        continue;
                    }
                    visited.insert(key);
                    let seen = [fft || dest == "fft", dac || dest == "dac"];
                    count += dfs(adj, dest, visited, memo, seen);
                    visited.remove(key);
                }
            }
            memo.insert(entry, count);
            count
        }
        let mut visited = HashSet::new();
        visited.insert(start);
        let seen = if constrained {
            [false, false]
        } else {
            [true, true]
        };
        dfs(&self.adj, start, &mut visited, &mut HashMap::new(), seen)
    }
}

#[cfg(test)]
mod test {
    use super::Day11;
    use aoc_lib::{answer::Answer, input, solution::Solution};

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_11_test_a.txt", crate::FILES_PREFIX_TEST)).unwrap();
        assert_eq!(<i64 as Into<Answer>>::into(5), Day11.part_a(&input));
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_11_test_b.txt", crate::FILES_PREFIX_TEST)).unwrap();
        assert_eq!(<i64 as Into<Answer>>::into(2), Day11.part_b(&input));
    }
}
