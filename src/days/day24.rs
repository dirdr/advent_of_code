use z3::{
    ast::{Ast, Bool, Int},
    Config, Context, Solver,
};

use crate::helper_lib::{answer::Answer, solution::Solution, vec2::Vec2};
use core::ops::RangeInclusive;
use std::ops::Sub;

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
    let mut count = 0;
    for i in 0..hailstones.len() {
        let hailstone = &hailstones[i];
        let u = HailstonePath::from(&hailstone);
        for j in (i + 1)..hailstones.len() {
            let other = &hailstones[j];
            let v = HailstonePath::from(&other);

            // will never inteserct
            if u.slope == v.slope {
                continue;
            }

            let x_int = u.intersection(&v);
            let y_int = u.evaluate(x_int);

            let t_curr = (x_int - hailstone.position.0 as f32) / hailstone.velocity.0 as f32;
            let t_other = (x_int - other.position.0 as f32) / other.velocity.0 as f32 as f32;

            // println!("{:?}", range);
            if range.contains(&x_int) && range.contains(&y_int) && t_curr > 0.0 && t_other > 0.0 {
                // println!("time of intersection {}", t);
                // println!("x : {}", hailstone.position.0);
                // println!("x_b : {}", other.position.0);
                // println!("{x_int}");
                // println!("{y_int}");
                // println!("");
                count += 1;
            }
        }
    }
    count
}

fn solve_b(hailstones: &[Hailstone]) -> i64 {
    let config = Config::new();
    let ctx = Context::new(&config);
    let solver = Solver::new(&ctx);

    let (xr, yr, zr) = (
        z3::ast::Int::new_const(&ctx, "xr"),
        z3::ast::Int::new_const(&ctx, "yr"),
        z3::ast::Int::new_const(&ctx, "zr"),
    );

    let (vxr, vyr, vzr) = (
        z3::ast::Int::new_const(&ctx, "vxr"),
        z3::ast::Int::new_const(&ctx, "vyr"),
        z3::ast::Int::new_const(&ctx, "vzr"),
    );

    for i in 0..hailstones.len() {
        let hailstone = &hailstones[i];

        let (xh, yh, zh) = (
            Int::from_i64(&ctx, hailstone.position.0 as i64),
            Int::from_i64(&ctx, hailstone.position.1 as i64),
            Int::from_i64(&ctx, hailstone.position.2 as i64),
        );

        let (vxh, vyh, vzh) = (
            Int::from_i64(&ctx, hailstone.velocity.0 as i64),
            Int::from_i64(&ctx, hailstone.velocity.1 as i64),
            Int::from_i64(&ctx, hailstone.velocity.2 as i64),
        );

        let t = Int::new_const(
            &ctx,
            format!(
                "t{}{}{}",
                hailstone.position.0, hailstone.position.1, hailstone.position.2
            ),
        );

        solver.assert(&((&vxr - &vxh) * &t)._eq(&(&xh - &xr)));
        solver.assert(&((&vyr - &vyh) * &t)._eq(&(&yh - &yr)));
        solver.assert(&((&vzr - &vzh) * &t)._eq(&(&zh - &zr)));
    }

    match solver.check() {
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let xr_value = model.eval(&xr, false).unwrap();
            let yr_value = model.eval(&yr, false).unwrap();
            let zr_value = model.eval(&zr, false).unwrap();
            xr_value.as_i64().unwrap() + yr_value.as_i64().unwrap() + zr_value.as_i64().unwrap()
        }
        z3::SatResult::Unsat => {
            println!("Unsat !");
            0
        }
        z3::SatResult::Unknown => {
            println!("Unknown !");
            0
        }
    }
}

fn parse(input: &[String]) -> Vec<Hailstone> {
    let mut hailstones = vec![];
    let mut id = 0;
    for line in input {
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
        id += 1;
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

    // return the point x where the two paths intersect
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
    // for z3 bindings, as if a const have the same name, it cause the model to be unsat...
    // used to name the `t` of collision in the part 2
    id: usize,
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
