use crate::helper_lib::{answer::Answer, directions::Direction, solution::Solution, vec2::Vec2};

pub struct Day18;

impl Solution for Day18 {
    fn part_a(&self, input: &[String]) -> Answer {
        let plan = parse_a(input);
        solve(plan.instructions).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let plan = parse_b(input);
        solve(plan.instructions).into()
    }
}

fn solve(instructions: Vec<Instruction>) -> isize {
    let mut pos = Vec2::new(0_isize, 0_isize);
    let mut perimeter = 0;
    let mut area = 0;
    for instruction in instructions {
        let step = instruction.len as isize;
        perimeter += step;
        let after = instruction.direction.to_offset() * step;
        pos += after;
        area += pos.x * after.y;
    }
    // number of lava cubes in the trench = inside points + outside points
    // 1) inside points = pick's theorem (I = A - P/2 + 1)
    // 2) outside points = P
    // <=> number of lava cubes = A + P/2 + 1
    area + perimeter / 2 + 1
}

#[derive(Debug)]
struct DigPlan {
    instructions: Vec<Instruction>,
}

fn parse_a(input: &[String]) -> DigPlan {
    let mut instructions = vec![];
    for line in input {
        let content = line.split_whitespace().collect::<Vec<_>>();
        let instruction = Instruction {
            direction: to_direction_a(content[0].chars().next().unwrap()),
            len: content[1].parse::<usize>().unwrap(),
        };
        instructions.push(instruction);
    }
    DigPlan { instructions }
}

fn parse_b(input: &[String]) -> DigPlan {
    let mut instructions = vec![];
    for line in input {
        let content = line.split_whitespace().collect::<Vec<_>>()[2]
            .trim_start_matches("(#")
            .trim_end_matches(')');
        let len = usize::from_str_radix(&content[0..5], 16).unwrap();
        let direction = to_direction_b(content.chars().nth(5).unwrap());
        instructions.push(Instruction { direction, len })
    }
    DigPlan { instructions }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    direction: Direction,
    len: usize,
}

fn to_direction_a(ch: char) -> Direction {
    match ch {
        'R' => Direction::East,
        'D' => Direction::South,
        'U' => Direction::North,
        'L' => Direction::West,
        _ => unreachable!(),
    }
}

fn to_direction_b(ch: char) -> Direction {
    match ch {
        '0' => Direction::East,
        '1' => Direction::South,
        '2' => Direction::West,
        '3' => Direction::North,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::Day18;

    #[test]
    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_18_test.txt",
            helper_lib::consts::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day18.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(62), answer);
    }

    #[test]
    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_18_test.txt",
            helper_lib::consts::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day18.part_b(&input);
        assert_eq!(<i64 as Into<Answer>>::into(952408144115), answer);
    }
}
