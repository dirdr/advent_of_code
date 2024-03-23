use itertools::Itertools;
use z3::{
    ast::{Ast, Int},
    Config, Context, SatResult, Solver,
};

use crate::helper_lib::{answer::Answer, solution::Solution, vec2::Vec2};
use core::ops::RangeInclusive;

pub struct Day24;

impl Solution for Day24 {
    fn part_a(&self, input: &[String]) -> Answer {
        let hailstones = parse(input);
        solve_a(&hailstones, 200000000000000.0..=400000000000000.0).into()
    }

    fn part_b(&self, input: &[String]) -> Answer {
        let hailstones = parse(input);
        solve_b(&hailstones).into()
    }
}

fn solve_a(hailstones: &[Hailstone], range: RangeInclusive<f32>) -> usize {
    hailstones
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| a.collision_point(b))
        .filter(|&pos| range.contains(&pos.x) && range.contains(&pos.y))
        .count()
}

fn solve_b(hailstones: &[Hailstone]) -> i64 {
    let config = Config::new();
    let ctx = Context::new(&config);
    let solver = Solver::new(&ctx);

    let (xr, yr, zr, vxr, vyr, vzr) = (
        Int::new_const(&ctx, "xr"),
        Int::new_const(&ctx, "yr"),
        Int::new_const(&ctx, "zr"),
        Int::new_const(&ctx, "vxr"),
        Int::new_const(&ctx, "vyr"),
        Int::new_const(&ctx, "vzr"),
    );

    for hailstone in hailstones {
        let (xh, yh, zh, vxh, vyh, vzh, t) = (
            Int::from_i64(&ctx, hailstone.position.0 as i64),
            Int::from_i64(&ctx, hailstone.position.1 as i64),
            Int::from_i64(&ctx, hailstone.position.2 as i64),
            Int::from_i64(&ctx, hailstone.velocity.0 as i64),
            Int::from_i64(&ctx, hailstone.velocity.1 as i64),
            Int::from_i64(&ctx, hailstone.velocity.2 as i64),
            Int::new_const(&ctx, format!("{}", hailstone.id)),
        );
        solver.assert(&((&vxr - &vxh) * &t)._eq(&(&xh - &xr)));
        solver.assert(&((&vyr - &vyh) * &t)._eq(&(&yh - &yr)));
        solver.assert(&((&vzr - &vzh) * &t)._eq(&(&zh - &zr)));
    }

    // The problem has at least one solution.
    assert!(solver.check() == SatResult::Sat);

    let model = solver.get_model().unwrap();
    let xr_value = model.eval(&xr, false).unwrap();
    let yr_value = model.eval(&yr, false).unwrap();
    let zr_value = model.eval(&zr, false).unwrap();
    xr_value.as_i64().unwrap() + yr_value.as_i64().unwrap() + zr_value.as_i64().unwrap()
}

fn parse(input: &[String]) -> Vec<Hailstone> {
    let mut hailstones = vec![];
    for (id, line) in input.iter().enumerate() {
        let (position, velocity) = line.split_once('@').unwrap();
        let position = position
            .split(',')
            .map(|p| p.trim().parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let velocity = velocity
            .split(',')
            .map(|p| p.trim().parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        hailstones.push(Hailstone {
            position: (position[0], position[1], position[2]),
            velocity: (velocity[0], velocity[1], velocity[2]),
            id,
        });
    }
    hailstones
}

struct HailstonePath {
    slope: f32,
    intercept: f32,
}

impl HailstonePath {
    fn from(hailstone: &Hailstone) -> Self {
        let n = Vec2::new(hailstone.position.0, hailstone.position.1);
        let velocity = Vec2::new(hailstone.velocity.0, hailstone.velocity.1);
        let m = n + velocity;
        let slope = (m.y - n.y) as f32 / (m.x - n.x) as f32;
        let intercept = n.y as f32 - (slope * n.x as f32);
        Self { slope, intercept }
    }

    // return the x value where the two paths intersect
    fn intersection(&self, other: &HailstonePath) -> f32 {
        (other.intercept - self.intercept) / (self.slope - other.slope)
    }

    fn evaluate(&self, x: f32) -> f32 {
        self.slope * x + self.intercept
    }
}

#[derive(PartialEq)]
struct Hailstone {
    position: (isize, isize, isize),
    // velocity is given as (dx, dy, dz) each unit of time.
    velocity: (isize, isize, isize),
    // for z3 bindings, if a const have the same name it cause the model to be unsat...
    // because the internal representation of the consts might depend on the string named passed to
    // new_consts(). Used to name the const `t` (time of collision) for every hailstone
    id: usize,
}

impl Hailstone {
    fn collision_point(&self, other: &Hailstone) -> Option<Vec2<f32>> {
        let u = HailstonePath::from(self);
        let v = HailstonePath::from(other);

        // will never inteserct
        if u.slope == v.slope {
            return None;
        }

        let x_int = u.intersection(&v);
        let y_int = u.evaluate(x_int);

        let t_curr = (x_int - self.position.0 as f32) / self.velocity.0 as f32;
        let t_other = (x_int - other.position.0 as f32) / other.velocity.0 as f32;

        // intersect in the past
        if t_curr <= 0.0 || t_other <= 0.0 {
            return None;
        }

        Some(Vec2::new(x_int, y_int))
    }
}

#[cfg(test)]
mod test {
    use crate::helper_lib::{self, answer::Answer, input, solution::Solution};

    use super::{parse, solve_a, Day24};

    #[test]
    fn test_a() {
        let input = input::read_file(&format!(
            "{}day_24_test.txt",
            helper_lib::consts::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = solve_a(&parse(&input), 7.0..=27.0).into();
        assert_eq!(<i32 as Into<Answer>>::into(2), answer);
    }

    #[test]
    fn test_b() {
        let input = input::read_file(&format!(
            "{}day_24_test.txt",
            helper_lib::consts::FILES_PREFIX_TEST
        ))
        .unwrap();
        let answer = Day24.part_b(&input);
        assert_eq!(<i32 as Into<Answer>>::into(47), answer);
    }
}
