use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

use aoc_lib::{answer::Answer, solution::Solution};

pub struct Day8;

impl Solution for Day8 {
    fn part_a(&self, input: &[String]) -> Answer {
        let mut playgound = Playground::from_input(input);
        playgound.process_n(1000);
        playgound.part_a().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let mut playgound = Playground::from_input(input);
        playgound.part_b().into()
    }
}

struct Playground {
    find: UnionFind,
    distance_heap: BinaryHeap<Reverse<(usize, usize, usize)>>,
    idx_to_circuit: HashMap<usize, Circuit>,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Circuit {
    pos: (usize, usize, usize),
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    heap: BinaryHeap<(usize, usize)>,
    current: HashMap<usize, usize>,
}

impl Playground {
    fn from_input(input: &[String]) -> Self {
        let n = input.len();
        let mut idx_to_circuit = HashMap::with_capacity(n);
        let mut distance_heap = BinaryHeap::new();

        for (i, line) in input.iter().enumerate() {
            let coords: Vec<usize> = line.split(',').map(|s| s.parse().unwrap()).collect();

            idx_to_circuit.insert(
                i,
                Circuit {
                    pos: (coords[0], coords[1], coords[2]),
                },
            );
        }

        for a in 0..n {
            for b in (a + 1)..n {
                let dist = idx_to_circuit[&a].euclidean_distance(&idx_to_circuit[&b]);
                distance_heap.push(Reverse((dist, a, b)));
            }
        }

        Self {
            idx_to_circuit,
            find: UnionFind::new(n),
            distance_heap,
        }
    }

    fn process_n(&mut self, n: usize) {
        for _ in 0..n {
            if let Some(Reverse((_, a, b))) = self.distance_heap.pop()
                && self.find.find(a) != self.find.find(b)
            {
                self.find.union(a, b);
            }
        }
    }

    fn part_b(&mut self) -> usize {
        let (mut last_a, mut last_b) = (0, 0);
        while let Some(Reverse((_, a, b))) = self.distance_heap.pop() {
            if self.find.find(a) != self.find.find(b) {
                self.find.union(a, b);
                last_a = a;
                last_b = b;
            }
        }
        self.idx_to_circuit[&last_a].pos.0 * self.idx_to_circuit[&last_b].pos.0
    }

    fn part_a(&mut self) -> usize {
        let top = self.find.top_k(3);
        top.iter().map(|(_, size)| size).product()
    }
}

impl Circuit {
    fn euclidean_distance(&self, other: &Circuit) -> usize {
        let dx = (other.pos.0 as isize - self.pos.0 as isize).abs();
        let dy = (other.pos.1 as isize - self.pos.1 as isize).abs();
        let dz = (other.pos.2 as isize - self.pos.2 as isize).abs();

        (dx * dx + dy * dy + dz * dz) as usize
    }
}

impl Display for Circuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.pos.0, self.pos.1, self.pos.2)
    }
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        let mut heap = BinaryHeap::with_capacity(size);
        let mut current = HashMap::with_capacity(size);

        for i in 0..size {
            heap.push((1, i));
            current.insert(i, 1);
        }

        UnionFind {
            parent: (0..size).collect(),
            size: vec![1; size],
            heap,
            current,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);

        if root_a == root_b {
            return;
        }

        if self.size[root_a] < self.size[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }

        self.current.remove(&root_a);
        self.current.remove(&root_b);

        self.parent[root_b] = root_a;
        self.size[root_a] += self.size[root_b];

        let new_size = self.size[root_a];
        self.current.insert(root_a, new_size);
        self.heap.push((new_size, root_a));
    }

    pub fn top_k(&mut self, k: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::with_capacity(k);

        while result.len() < k {
            match self.heap.pop() {
                Some((size, root)) if self.current.get(&root) == Some(&size) => {
                    result.push((root, size));
                }
                None => break,
                _ => continue,
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use crate::day08::Playground;

    use super::Day8;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_08_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let mut playground = Playground::from_input(&input);
        playground.process_n(10);
        assert_eq!(<i32 as Into<Answer>>::into(40), playground.part_a().into());
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_08_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day8.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(25272), answer);
    }
}
