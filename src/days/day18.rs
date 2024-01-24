use crate::helper_lib::{answer::Answer, directions::Direction, solution::Solution, vec2::Vec2};

pub struct Day18;

impl Solution for Day18 {
    fn part_a(&self, input: &[String]) -> Answer {
        let plan = parse(input);
        let trench = plan.dig();
        trench.count().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        todo!()
    }
}

#[derive(Debug)]
struct DigPlan<'a> {
    instructions: Vec<Instruction<'a>>,
}

#[derive(Debug)]
struct Trench {
    vertices: Vec<Vec2<isize>>,
}

impl Trench {
    fn count(&self) -> i32 {
        let len = self.vertices.len();
        let area = self
            .vertices
            .iter()
            .enumerate()
            .fold(0i32, |acc, (i, p)| {
                let l = (i + 1) % len;
                acc + (p.x as i32 * self.vertices[l].y as i32
                    - p.y as i32 * self.vertices[l].x as i32)
            })
            .abs()
            / 2;
        (area - (len as i32 / 2) + 1) as i32
    }
}

impl<'a> DigPlan<'a> {
    fn dig(&self) -> Trench {
        // let mut color_map = HashMap::new();
        let path = self.calculate_path();
        let vertices = self.walk(&path);
        Trench { vertices }
    }

    fn walk(&self, path: &[Vec2<isize>]) -> Vec<Vec2<isize>> {
        let mut vertices = vec![];
        for i in 0..path.len() - 1 {
            let (start, end) = (path[i], path[i + 1]);
            if start.x == end.x {
                for y in start.y.min(end.y)..=start.y.max(end.y) {
                    let pos = Vec2::new(start.x, y);
                    vertices.push(pos);
                }
            } else {
                for x in start.x.min(end.x)..=start.x.max(end.x) {
                    let pos = Vec2::new(x, start.y);
                    vertices.push(pos);
                }
            }
        }
        vertices
    }

    fn calculate_path(&self) -> Vec<Vec2<isize>> {
        self.instructions.iter().fold(
            vec![Vec2::new(0_isize, 0_isize)],
            |mut path, instruction| {
                let new_pos = instruction.calculate_new_pos(path.last().unwrap().to_owned());
                path.push(new_pos);
                path
            },
        )
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction<'a> {
    direction: Direction,
    len: usize,
    color: &'a str,
}

impl<'a> Instruction<'a> {
    fn calculate_new_pos(&self, pos: Vec2<isize>) -> Vec2<isize> {
        let len = self.len as isize;
        match self.direction {
            Direction::North => pos + Vec2::new(0, -len),
            Direction::South => pos + Vec2::new(0, len),
            Direction::East => pos + Vec2::new(len, 0),
            Direction::West => pos + Vec2::new(-len, 0),
        }
    }
}

fn parse(input: &[String]) -> DigPlan {
    let mut instructions = vec![];
    for line in input {
        let content = line.split_whitespace().collect::<Vec<_>>();
        let instruction = Instruction {
            direction: to_direction(content[0].chars().next().unwrap()),
            len: content[1].parse::<usize>().unwrap(),
            color: content[2],
        };
        instructions.push(instruction);
    }
    DigPlan { instructions }
}

fn to_direction(ch: char) -> Direction {
    match ch {
        'R' => Direction::East,
        'D' => Direction::South,
        'U' => Direction::North,
        'L' => Direction::West,
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
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day18.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(62), answer);
    }

    #[test]
    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_18_test.txt",
            helper_lib::consts::FILES_PREFIX
        ))
        .unwrap();
        let answer = Day18.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(0), answer);
    }
}
