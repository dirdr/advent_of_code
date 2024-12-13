use aoc_lib::{answer::Answer, solution::Solution, vec2::Vec2};

pub struct Day13;

impl Solution for Day13 {
    fn part_a(&self, input: &[String]) -> Answer {
        let mut machines = parse(input);
        solve(&mut machines, Some(100), 0).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let mut machines = parse(input);
        solve(&mut machines, None, 10000000000000).into()
    }
}

fn solve(machines: &mut [ClawMachine], press_limit: Option<usize>, shift: u64) -> u64 {
    let mut total = 0;
    for machine in machines.iter_mut() {
        machine.prize += Vec2::new(shift, shift);
        if let Some(tokens) = machine.solve(press_limit) {
            total += tokens;
        }
    }
    total
}

struct ClawMachine {
    a: Vec2<u64>,
    b: Vec2<u64>,
    prize: Vec2<u64>,
}

impl ClawMachine {
    /// We know from the problem statement than:
    /// ka * ax + kb * bx = px
    /// ka * ay + kb * by = py
    /// So we isolate the expression of ka in the first equation, and then plug it into the second
    /// equation.
    /// Then we solve and see if the solution work (because the solution should be integers and it
    /// is simpler to check than to handle floats -> integer casts and checks).
    fn solve(&self, press_limit: Option<usize>) -> Option<u64> {
        let (ax, bx, ay, by, px, py) = (
            self.a.x as i64,
            self.b.x as i64,
            self.a.y as i64,
            self.b.y as i64,
            self.prize.x as i64,
            self.prize.y as i64,
        );

        let denom = by * ax - bx * ay;

        if denom == 0 {
            return None;
        }

        let kb = (ax * py - px * ay).checked_div(denom)?;
        let ka = (px - kb * bx).checked_div(ax)?;

        if ka < 0 || kb < 0 {
            return None;
        }

        if let Some(limit) = press_limit {
            if ka > limit as i64 || kb > limit as i64 {
                return None;
            }
        }

        if ka * ax + kb * bx != px || ka * ay + kb * by != py {
            return None;
        }

        Some(ka as u64 * 3 + kb as u64)
    }
}

fn parse(input: &[String]) -> Vec<ClawMachine> {
    let mut machines = vec![];
    for machine in input
        .split(|line| line.is_empty())
        .filter(|&g| !g.is_empty())
        .map(|g| g.to_vec())
        .collect::<Vec<Vec<String>>>()
    {
        let (ax, ay) = parse_button(&machine[0]);
        let (bx, by) = parse_button(&machine[1]);

        let (_, p) = machine[2].split_once(": ").unwrap();
        let (px, py) = p.trim().split_once(", ").unwrap();
        let (px, py) = (
            px[2..].parse::<u64>().unwrap(),
            py[2..].parse::<u64>().unwrap(),
        );

        machines.push(ClawMachine {
            a: Vec2::new(ax, ay),
            b: Vec2::new(bx, by),
            prize: Vec2::new(px, py),
        })
    }
    machines
}

fn parse_button(button: &str) -> (u64, u64) {
    let (_, b) = button.split_once(": ").unwrap();
    let (bx, by) = b.split_once(", ").unwrap();
    let (bx, by) = (
        bx[2..].parse::<u64>().unwrap(),
        by[2..].parse::<u64>().unwrap(),
    );
    (bx, by)
}

#[cfg(test)]
mod test {
    use aoc_lib::{answer::Answer, input, solution::Solution};

    use super::Day13;

    #[test]
    fn test_a() {
        let input =
            input::read_file(&format!("{}day_13_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day13.part_a(&input);
        assert_eq!(<i32 as Into<Answer>>::into(480), answer);
    }

    #[test]
    fn test_b() {
        let input =
            input::read_file(&format!("{}day_13_test.txt", crate::FILES_PREFIX_TEST)).unwrap();
        let answer = Day13.part_b(&input);
        assert_eq!(<u64 as Into<Answer>>::into(875318608908), answer);
    }
}
