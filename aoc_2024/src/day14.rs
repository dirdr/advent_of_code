use aoc_lib::{answer::Answer, matrix::Matrix, solution::Solution, vec2::Vec2};

pub struct Day14;

impl Solution for Day14 {
    fn part_a(&self, input: &[String]) -> Answer {
        Map::from_input(input).simulate(100).count_safety().into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let mut map = Map::from_input(input);
        let mut curr = 1;
        loop {
            map = map.simulate(1);
            // If we find `consecutive_threshold` robot occupied cell in the figure, we might have a pattern
            if map.detect_tree(20) {
                return curr.into();
            }
            curr += 1;
        }
    }
}

struct Map {
    bound: Vec2<i32>,
    robots: Vec<Robot>,
    map: Matrix<bool>,
}

struct Robot {
    pos: Vec2<i32>,
    vel: Vec2<i32>,
}

impl Map {
    fn from_input(input: &[String]) -> Self {
        let mut robots = vec![];
        for line in input {
            robots.push(Robot::from_description(line));
        }
        let bound = Vec2::new(
            robots.iter().map(|r| r.pos.x).max().unwrap() + 1,
            robots.iter().map(|r| r.pos.y).max().unwrap() + 1,
        );
        let map = Matrix::new(bound.y as usize, bound.x as usize, false);
        Self { robots, bound, map }
    }

    fn simulate(mut self, iterations: i32) -> Self {
        for robot in self.robots.iter_mut() {
            self.map[Vec2::<usize>::try_from(robot.pos).unwrap()] = false;
            robot.pos += robot.vel * iterations;
            robot.pos.x = robot.pos.x.rem_euclid(self.bound.x);
            robot.pos.y = robot.pos.y.rem_euclid(self.bound.y);
            self.map[Vec2::<usize>::try_from(robot.pos).unwrap()] = true;
        }
        self
    }

    fn count_safety(&self) -> usize {
        let half_bounds = self.bound / 2;
        let mut quadrants = [0; 4];
        for pos in self.robots.iter().map(|r| r.pos) {
            if pos.x == half_bounds.x || pos.y == half_bounds.y {
                continue;
            }
            let width = (0..=half_bounds.x).contains(&pos.x);
            let height = (0..=half_bounds.y).contains(&pos.y);
            quadrants[((width as usize) << 1) | height as usize] += 1;
        }
        quadrants.iter().product()
    }

    fn detect_tree(&self, consecutive_threshold: usize) -> bool {
        let mut max = 0;
        for y in 0..self.map.rows {
            let mut cons = 0;
            for x in 1..self.map.cols {
                let pos = Vec2::new(x, y);
                let prev = Vec2::new(x - 1, y);
                if self.map[pos] && self.map[prev] {
                    cons += 1;
                }
            }
            max = max.max(cons);
        }
        max >= consecutive_threshold
    }
}

impl Robot {
    fn from_description(desc: &str) -> Self {
        let (p, v) = desc.split_once(' ').unwrap();
        let (px, py) = p[2..].split_once(',').unwrap();
        let (vx, vy) = v[2..].split_once(',').unwrap();
        let (px, py) = (px.parse::<i32>().unwrap(), py.parse::<i32>().unwrap());
        let (vx, vy) = (vx.parse::<i32>().unwrap(), vy.parse::<i32>().unwrap());
        Self {
            pos: Vec2::new(px, py),
            vel: Vec2::new(vx, vy),
        }
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day14;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_14_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day14.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(12), answer);
    }
}
