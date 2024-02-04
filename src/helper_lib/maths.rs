use super::vec2::Vec2;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

fn divided_differences(points: &[Vec2<f64>]) -> Vec<f64> {
    let mut differences = points.iter().map(|p| p.y).collect::<Vec<_>>();
    let n = points.len();
    for i in 1..n {
        for j in (i..n).rev() {
            differences[j] =
                (differences[j] - differences[j - 1]) / (points[j].x - points[j - i].x);
        }
    }
    differences
}

pub fn newtonian_iterp(points: &[Vec2<usize>]) -> impl Fn(f64) -> f64 {
    let points = points
        .into_iter()
        .map(|&p| Vec2::<f64>::from(p))
        .collect::<Vec<_>>();
    let coefficiants = divided_differences(&points);
    let xs = points.iter().map(|p| p.x).collect::<Vec<_>>();
    move |x: f64| -> f64 {
        let mut result = coefficiants[0];
        let mut product = 1.0;
        for (i, &xi) in xs.iter().enumerate().take(coefficiants.len() - 1) {
            product *= x - xi;
            result += coefficiants[i + 1] * product;
        }
        result
    }
}
