use aoc_lib::{answer::Answer, solution::Solution};

pub struct Day1;

impl Solution for Day1 {
    fn part_a(&self, input: &[String]) -> Answer {
        let mut problem = Problem::parse(input);
        problem.execute_instructions(Protocol::End).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let mut problem = Problem::parse(input);
        problem.execute_instructions(Protocol::Any).into()
    }
}

struct Problem {
    dial: Dial,
    instructions: Vec<(Direction, usize)>,
}

impl Problem {
    fn parse(input: &[String]) -> Problem {
        let instructions = input
            .iter()
            .map(|l| {
                let (direction, amount) = l.split_at(1);
                let direction = Direction::from(direction);
                let amount = amount.parse::<usize>().unwrap();
                (direction, amount)
            })
            .collect::<Vec<_>>();
        let dial = Dial::new();
        Self { dial, instructions }
    }

    fn execute_instructions(&mut self, protocol: Protocol) -> usize {
        let mut password = 0;

        for &(direction, amount) in &self.instructions {
            if protocol == Protocol::Any {
                let prev_pos = self.dial.position;

                let full_circles = amount / 100;
                password += full_circles;

                let remaining = amount % 100;

                let new_pos: isize = match direction {
                    Direction::Left => self.dial.position as isize - remaining as isize,
                    Direction::Right => self.dial.position as isize + remaining as isize,
                };

                if new_pos >= 100 || new_pos <= 0 {
                    let final_pos = new_pos.rem_euclid(100) as usize;
                    if final_pos != prev_pos && prev_pos != 0 {
                        password += 1;
                    }
                }
            }

            self.dial.rotate(direction, amount);

            if protocol == Protocol::End && self.dial.position == 0 {
                password += 1;
            }
        }

        password
    }
}

#[derive(PartialEq)]
enum Protocol {
    End,
    Any,
}

struct Dial {
    position: usize,
}

impl Dial {
    fn new() -> Dial {
        Self { position: 50 }
    }

    fn rotate(&mut self, direction: Direction, amount: usize) {
        match direction {
            Direction::Left => {
                self.position = (self.position as isize - amount as isize).rem_euclid(100) as usize
            }
            Direction::Right => {
                self.position = (self.position as isize + amount as isize).rem_euclid(100) as usize
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day1;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_01_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day1.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(3), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_01_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day1.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(6), answer);
    }
}
