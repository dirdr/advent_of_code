use aoc_lib::{answer::Answer, solution::Solution};

pub struct Day9;

impl Solution for Day9 {
    fn part_a(&self, input: &[String]) -> Answer {
        DiskMap::from_input(input)
            .expand()
            .compact(CompactingStrategy::Block)
            .checksum()
            .into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        DiskMap::from_input(input)
            .expand()
            .compact(CompactingStrategy::File)
            .checksum()
            .into()
    }
}

struct DiskMap {
    rep: Vec<usize>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum DiskBlock {
    Empty,
    FileBlock(usize),
}

struct ExpandedFormat {
    rep: Vec<DiskBlock>,
}

struct CompactFormat {
    rep: Vec<DiskBlock>,
}

enum CompactingStrategy {
    Block,
    File,
}

impl DiskMap {
    fn from_input(input: &[String]) -> Self {
        Self {
            rep: input[0]
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>(),
        }
    }

    fn expand(&self) -> ExpandedFormat {
        let mut rep = vec![];
        let mut current_file_id = 0;
        for i in 0..self.rep.len() {
            let block = if i % 2 == 0 {
                let fb = DiskBlock::FileBlock(current_file_id);
                current_file_id += 1;
                fb
            } else {
                DiskBlock::Empty
            };
            for _ in 0..self.rep[i] {
                rep.push(block);
            }
        }
        ExpandedFormat { rep }
    }
}

impl ExpandedFormat {
    fn compact(&self, strategy: CompactingStrategy) -> CompactFormat {
        match strategy {
            CompactingStrategy::Block => self.compact_block(),
            CompactingStrategy::File => self.compact_file(),
        }
    }

    fn compact_block(&self) -> CompactFormat {
        let mut rep = self.rep.clone();
        let mut back = self.rep.len() - 1;
        let mut front = 0;
        while front < back {
            while rep[back] == DiskBlock::Empty {
                back -= 1;
            }
            if rep[front] == DiskBlock::Empty {
                rep[front] = rep[back];
                rep[back] = DiskBlock::Empty;
            }
            front += 1;
        }
        CompactFormat { rep }
    }

    fn compact_file(&self) -> CompactFormat {
        let mut rep = self.rep.clone();

        let mut files = Vec::new();
        let mut i = 0;
        while i < rep.len() {
            if let DiskBlock::FileBlock(file_id) = rep[i] {
                let start = i;
                while i + 1 < rep.len() && rep[i + 1] == DiskBlock::FileBlock(file_id) {
                    i += 1;
                }
                let length = i - start + 1;
                files.push((file_id, start, length));
            }
            i += 1;
        }
        files.sort_by(|a, b| b.0.cmp(&a.0));

        for (file_id, file_start, file_len) in files {
            // Find a contiguous run of empty blocks large enough to hold the file
            // that is strictly to the left of file_start.
            let mut candidate_start = None;
            let mut free_run_start = 0;
            let mut free_run_length = 0;

            for (pos, &file) in rep.iter().enumerate().take(file_start) {
                if file == DiskBlock::Empty {
                    if free_run_length == 0 {
                        free_run_start = pos;
                    }
                    free_run_length += 1;
                    if free_run_length == file_len {
                        candidate_start = Some(free_run_start);
                        break;
                    }
                } else {
                    free_run_length = 0;
                }
            }
            if let Some(target_start) = candidate_start {
                for offset in 0..file_len {
                    rep[target_start + offset] = DiskBlock::FileBlock(file_id);
                    rep[file_start + offset] = DiskBlock::Empty;
                }
            }
        }
        CompactFormat { rep }
    }
}

impl CompactFormat {
    fn checksum(&self) -> usize {
        let mut pos = 0;
        self.rep.iter().fold(0, |acc, &x| {
            let add = match x {
                DiskBlock::FileBlock(id) => pos * id,
                DiskBlock::Empty => 0,
            };
            pos += 1;
            acc + add
        })
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day9;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_09_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day9.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(1928), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_09_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day9.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(2858), answer);
    }
}
