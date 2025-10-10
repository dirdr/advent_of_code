use std::collections::HashMap;

use aoc_lib::{answer::Answer, solution::Solution, vec2::Vec2};

pub struct Day21;

impl Solution for Day21 {
    fn part_a(&self, input: &[String]) -> Answer {
        solve(input, 3).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        solve(input, 26).into()
    }
}

fn solve(input: &[String], num_keypads: usize) -> i64 {
    let problem = Problem::from_input(input);
    let mut sum = 0i64;

    for code in problem.codes {
        let numerical = code
            .input
            .iter()
            .filter(|&&d| d.is_ascii_digit())
            .collect::<String>()
            .parse::<i64>()
            .unwrap();

        let mut num_keypad = Keypad::new(KeypadType::Numerical);
        let first_sequence = num_keypad.type_code(&code.input);

        let mut memo = HashMap::new();
        let final_length = calculate_length(&first_sequence, num_keypads - 1, &mut memo);

        sum += numerical * final_length;
    }

    sum
}

fn calculate_length(
    sequence: &[char],
    remaining_depth: usize,
    memo: &mut HashMap<(Vec<char>, usize), i64>,
) -> i64 {
    if remaining_depth == 0 {
        return sequence.len() as i64;
    }

    let key = (sequence.to_vec(), remaining_depth);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let mut total_length = 0i64;
    let mut current_pos = 'A';

    for &target_button in sequence {
        let path = get_directional_path(current_pos, target_button);

        total_length += calculate_length(&path, remaining_depth - 1, memo);

        current_pos = target_button;
    }

    memo.insert(key, total_length);
    total_length
}

fn get_directional_path(from: char, to: char) -> Vec<char> {
    if from == to {
        return vec!['A'];
    }

    let pos_map = |c| match c {
        '^' => Vec2::new(1, 0),
        'A' => Vec2::new(2, 0),
        '<' => Vec2::new(0, 1),
        'v' => Vec2::new(1, 1),
        '>' => Vec2::new(2, 1),
        _ => unreachable!(),
    };

    let from_pos = pos_map(from);
    let to_pos = pos_map(to);

    let mut path = generate_safe_path(from_pos, to_pos, false);
    path.push('A');
    path
}

fn generate_safe_path(pos: Vec2<usize>, target: Vec2<usize>, is_numerical: bool) -> Vec<char> {
    if pos == target {
        return vec![];
    }

    let gap_pos = if is_numerical {
        Vec2::new(0, 3)
    } else {
        Vec2::new(0, 0)
    };

    let dx = target.x as isize - pos.x as isize;
    let dy = target.y as isize - pos.y as isize;

    let mut moves = Vec::new();

    let h_first_safe = !(pos.y == gap_pos.y && target.x == gap_pos.x);
    let v_first_safe = !(pos.x == gap_pos.x && target.y == gap_pos.y);

    if h_first_safe && (dx < 0 || !v_first_safe) {
        for _ in 0..dx.abs() {
            moves.push(if dx > 0 { '>' } else { '<' });
        }
        for _ in 0..dy.abs() {
            moves.push(if dy > 0 { 'v' } else { '^' });
        }
    } else {
        for _ in 0..dy.abs() {
            moves.push(if dy > 0 { 'v' } else { '^' });
        }
        for _ in 0..dx.abs() {
            moves.push(if dx > 0 { '>' } else { '<' });
        }
    }

    moves
}

#[derive(Clone)]
enum KeypadType {
    Numerical,
}

impl KeypadType {
    fn get_arm_initial_pos(&self) -> Vec2<usize> {
        match self {
            KeypadType::Numerical => Vec2::new(2, 3),
        }
    }

    fn key_to_pos(&self, key: char) -> Vec2<usize> {
        match self {
            KeypadType::Numerical => match key {
                '0' => Vec2::new(1, 3),
                '1' => Vec2::new(0, 2),
                '2' => Vec2::new(1, 2),
                '3' => Vec2::new(2, 2),
                '4' => Vec2::new(0, 1),
                '5' => Vec2::new(1, 1),
                '6' => Vec2::new(2, 1),
                '7' => Vec2::new(0, 0),
                '8' => Vec2::new(1, 0),
                '9' => Vec2::new(2, 0),
                'A' => Vec2::new(2, 3),
                _ => unreachable!(),
            },
        }
    }
}

#[derive(Debug)]
struct Code {
    input: Vec<char>,
}

struct Problem {
    codes: Vec<Code>,
}

#[derive(Clone)]
struct Keypad {
    arm_initial_pos: Vec2<usize>,
    keypad_type: KeypadType,
    cache: HashMap<(Vec2<usize>, Vec2<usize>), Vec<char>>,
}

impl Keypad {
    fn new(keypad_type: KeypadType) -> Self {
        Self {
            arm_initial_pos: keypad_type.get_arm_initial_pos(),
            keypad_type,
            cache: HashMap::new(),
        }
    }

    fn shortest_path_inputs(&mut self, pos: Vec2<usize>, target: Vec2<usize>) -> Vec<char> {
        if let Some(cached) = self.cache.get(&(pos, target)) {
            return cached.clone();
        }

        let path = generate_safe_path(
            pos,
            target,
            matches!(self.keypad_type, KeypadType::Numerical),
        );
        self.cache.insert((pos, target), path.clone());
        path
    }

    fn type_code(&mut self, code: &Vec<char>) -> Vec<char> {
        let mut sequence = vec![];
        let mut pos = self.arm_initial_pos;
        for &token in code {
            let target = self.keypad_type.key_to_pos(token);
            sequence.extend_from_slice(&self.shortest_path_inputs(pos, target));
            sequence.push('A');
            pos = target;
        }
        sequence
    }
}

impl Problem {
    fn from_input(input: &[String]) -> Self {
        Problem {
            codes: input
                .iter()
                .map(|line| Code {
                    input: line.chars().collect::<Vec<_>>(),
                })
                .collect::<Vec<_>>(),
        }
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day21;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_21_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day21.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(126384), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_21_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day21.part_b(&input);
        assert_eq!(<i64 as Into<Answer>>::into(154115708116294), answer);
    }
}
