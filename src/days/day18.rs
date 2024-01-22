use crate::helper_lib::{
    answer::Answer, directions::Direction, matrix::Matrix, solution::Solution, vec2::Vec2,
};

pub struct Day18;

impl Solution for Day18 {
    fn part_a(&self, input: &[String]) -> Answer {
        let plan = parse(input);
        let trench = plan.dig();
        trench.into()
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
struct Trench<'a> {
    edges: Vec<Edge<'a>>,
}

#[derive(Debug)]
struct Edge<'a> {
    vertices: Vec<Vec2<usize>>,
    color: &'a str,
}

impl<'a> Edge<'a> {
    fn new(vertices: Vec<Vec2<usize>>, color: &'a str) -> Self {
        Self { vertices, color }
    }
}

impl<'a> DigPlan<'a> {
    fn dig(&self) -> Trench<'a> {
        let coordinates = self.trench_corner_coordinates();
        let size =
            Vec2::<usize>::try_from(coordinates[1] - coordinates[0]).unwrap() + Vec2::new(1, 1);
        let mut grid = Matrix::new(size.y, size.x, '.');
        let offset = coordinates[0].abs();
        let path = self.calculate_edges(offset);
        println!("coordinates {:?}", coordinates);
        println!("offset {:?}", offset);
        for p in path.iter() {
            println!("{:?}", p);
        }
        Trench {
            edges: self.walk(&mut grid, &path),
        }
    }

    fn walk(&self, grid: &mut Matrix<char>, path: &[(Vec2<isize>, &'a str)]) -> Vec<Edge<'a>> {
        let mut edges = vec![];
        for i in 0..path.len() - 1 {
            let (start, end) = (path[i].0, path[i + 1].0);
            let mut vertices = vec![];
            if start.x == end.x {
                for y in start.y.min(end.y)..=start.y.max(end.y) {
                    let pos = Vec2::new(start.x as usize, y as usize);
                    vertices.push(pos);
                }
            } else {
                for x in start.x.min(end.x)..=start.x.max(end.x) {
                    let pos = Vec2::new(x as usize, start.y as usize);
                    vertices.push(pos);
                }
            }
            let edge = Edge {
                vertices,
                color: path[i + 1].1,
            };
            edges.push(edge);
        }
        edges
    }

    fn calculate_edges(&self, offset: Vec2<isize>) -> Vec<(Vec2<isize>, &'a str)> {
        let initial = self.instructions.iter().fold(
            vec![(Vec2::new(0_isize, 0_isize), "start")],
            |mut path, instruction| {
                let new_pos = instruction.calculate_new_pos(path.last().unwrap().to_owned().0);
                path.push((new_pos, instruction.color));
                path
            },
        );
        initial
            .iter()
            .map(|&inst| (inst.0 + offset, inst.1))
            .collect::<Vec<_>>()
    }
    // solution is 41019

    fn trench_corner_coordinates(&self) -> [Vec2<isize>; 2] {
        let mut max = Vec2::new(0_isize, 0_isize);
        let mut min = Vec2::new(isize::MAX, isize::MAX);
        let mut pos = Vec2::new(0, 0);
        for instruction in self.instructions.iter() {
            pos = instruction.calculate_new_pos(pos);
            max.x = max.x.max(pos.x);
            max.y = max.y.max(pos.y);
            min.x = min.x.min(pos.x);
            min.y = min.y.min(pos.y);
        }
        [min, max]
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
