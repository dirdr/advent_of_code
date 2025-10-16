use std::collections::{HashMap, HashSet};

use aoc_lib::{answer::Answer, solution::Solution};
use itertools::Itertools;

pub struct Day23;

impl Solution for Day23 {
    fn part_a(&self, input: &[String]) -> Answer {
        let network = Network::from_input(input);
        let triangles = network.all_triangles();
        triangles
            .iter()
            .filter(|&t| t.iter().any(|s| s.starts_with("t")))
            .count()
            .into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let network = Network::from_input(input);
        network.lan_password().into()
    }
}

struct Network {
    adj_list: Vec<HashSet<usize>>,
    idx_to_name: Vec<String>,
}

impl Network {
    fn from_input(input: &[String]) -> Self {
        let mut name_to_idx: HashMap<&str, usize> = HashMap::new();

        let mut all = HashSet::new();

        for line in input {
            let (a, b) = line.split_once('-').unwrap();
            all.insert(a);
            all.insert(b);
        }

        let mut idx_to_name = vec![String::from(""); all.len()];
        let mut adj_list: Vec<HashSet<usize>> = vec![HashSet::new(); all.len()];

        for (curr_idx, &name) in all.iter().enumerate() {
            name_to_idx.insert(name, curr_idx);
            idx_to_name[curr_idx] = name.to_string();
        }

        for line in input {
            let (a, b) = line.split_once('-').unwrap();
            let (&a, &b) = (name_to_idx.get(a).unwrap(), name_to_idx.get(b).unwrap());
            adj_list[a].insert(b);
            adj_list[b].insert(a);
        }

        Self {
            adj_list,
            idx_to_name,
        }
    }

    fn lan_password(&self) -> String {
        let mut cliques = vec![];
        let all_nodes = (0..self.adj_list.len()).collect::<HashSet<_>>();
        self.bron_kerbosch(HashSet::new(), all_nodes, HashSet::new(), &mut cliques);
        cliques
            .iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .iter()
            .map(|&e| self.idx_to_name[e].clone())
            .sorted_unstable()
            .join(",")
    }

    fn all_triangles(&self) -> Vec<[String; 3]> {
        let mut triangles = vec![];
        for u in 0..self.adj_list.len() {
            for &v in &self.adj_list[u] {
                if v > u {
                    for &w in &self.adj_list[u] {
                        if w > v && self.adj_list[v].contains(&w) {
                            triangles.push([
                                self.idx_to_name.get(u).unwrap().clone(),
                                self.idx_to_name.get(v).unwrap().clone(),
                                self.idx_to_name.get(w).unwrap().clone(),
                            ]);
                        }
                    }
                }
            }
        }
        triangles
    }

    fn bron_kerbosch(
        &self,
        r: HashSet<usize>,
        mut p: HashSet<usize>,
        mut x: HashSet<usize>,
        cliques: &mut Vec<Vec<usize>>,
    ) {
        if p.is_empty() && x.is_empty() {
            let mut clique: Vec<_> = r.into_iter().collect();
            clique.sort();
            cliques.push(clique);
            return;
        }

        let p_copy = p.clone();
        for v in p_copy {
            let mut r_new = r.clone();
            r_new.insert(v);

            let p_new: HashSet<_> = p.intersection(&self.adj_list[v]).copied().collect();
            let x_new: HashSet<_> = x.intersection(&self.adj_list[v]).copied().collect();

            self.bron_kerbosch(r_new, p_new, x_new, cliques);

            p.remove(&v);
            x.insert(v);
        }
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day23;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_23_test.txt", crate::FILES_PREFIX_TEST)).unwrap();

        let answer = Day23.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(7), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_23_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day23.part_b(&input);
        assert_eq!(
            <String as Into<Answer>>::into(String::from("co,de,ka,ta")),
            answer
        );
    }
}
