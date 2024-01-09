use crate::helper_lib::{answer::Answer, solution::Solution};

use std::fmt::{Debug, Display};

pub struct Day15;

impl Solution for Day15 {
    fn part_a(&self, input: &[String]) -> Answer {
        let parsed = input[0].split(',').collect::<Vec<&str>>();
        parsed.iter().map(|seq| hash(seq)).sum::<usize>().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let mut boxes = vec![Box::new(); 256];
        let sequences = parse(input);
        solve(sequences, &mut boxes);
        boxes
            .iter()
            .enumerate()
            .map(|(bn, b)| {
                b.lens
                    .iter()
                    .enumerate()
                    .map(|(sn, l)| (1 + bn) * (1 + sn) * l.focal_length)
                    .sum::<usize>()
            })
            .sum::<usize>()
            .into()
    }
}

fn solve<'a>(init: InitializationSequence<'a>, boxes: &mut [Box<'a>]) {
    for sequence in init.sequences {
        match sequence {
            Operation::Dash(label) => {
                let b = &mut boxes[hash(label)];
                b.operate_dash(label);
            }
            Operation::Equal(label, fl) => {
                let b = &mut boxes[hash(label)];
                b.operate_equal(label, fl);
            }
        }
    }
}

struct InitializationSequence<'a> {
    sequences: Vec<Operation<'a>>,
}

enum Operation<'a> {
    Dash(&'a str),
    Equal(&'a str, usize),
}

fn parse(input: &[String]) -> InitializationSequence {
    let mut sequences = vec![];
    let raws = input[0].split(',').collect::<Vec<&str>>();
    for raw_op in raws {
        if let Some((label, fl)) = raw_op.split_once('=') {
            sequences.push(Operation::Equal(label, fl.parse::<usize>().unwrap()));
        } else {
            let op = raw_op.trim_end_matches('-');
            sequences.push(Operation::Dash(op));
        }
    }
    InitializationSequence { sequences }
}

#[derive(Clone)]
struct Box<'a> {
    lens: Vec<Lens<'a>>,
}

impl<'a> Display for Box<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.lens.is_empty() {
            for l in self.lens.iter() {
                write!(f, "[{} {}]", l.label, l.focal_length)?;
            }
        }
        Ok(())
    }
}

impl<'a> Box<'a> {
    pub fn new() -> Self {
        Self { lens: vec![] }
    }
}

impl<'a> Box<'a> {
    fn operate_dash(&mut self, label: &str) {
        if let Some(i) = self.lens.iter().position(|l| l.label == label) {
            self.lens.remove(i);
        }
    }

    fn operate_equal(&mut self, label: &'a str, focal_length: usize) {
        if let Some(i) = self.lens.iter().position(|l| l.label == label) {
            self.lens[i] = Lens {
                label,
                focal_length,
            }
        } else {
            self.lens.push(Lens {
                label,
                focal_length,
            })
        }
    }
}

#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

fn hash(sequence: &str) -> usize {
    let mut hash = 0;
    for ch in sequence.chars() {
        let code = ch as u8;
        hash += code as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day15;

    #[test]
    pub fn test_a() {
        let input = input::read_file(&format!(
            "{}day_15_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day15.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(1320), answer);
    }

    #[test]
    pub fn test_b() {
        let input = input::read_file(&format!(
            "{}day_15_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day15.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(145), answer);
    }
}
