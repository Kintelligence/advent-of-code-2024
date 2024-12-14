pub fn solve_linear_diophantine(a: isize, b: isize, c: isize) -> Option<(isize, isize)> {
    let (x0, y0, gcd) = extended_euclid(a, b);

    if c % gcd != 0 {
        return None;
    }

    let scale = c / gcd;
    let mut x = x0 * scale;
    let mut y = y0 * scale;

    let a_gcd = a / gcd;
    let b_gcd = -b / gcd;

    let k = ((-x) as f64 / b_gcd as f64).ceil() as isize;
    x += k * b_gcd;
    y += k * a_gcd;

    if x > 0 && y > 0 {
        return Some((x, y));
    }

    None
}

pub fn extended_euclid(a: isize, b: isize) -> (isize, isize, isize) {
    if a == 0 {
        return (0, 1, b);
    }

    let (x1, y1, gcd) = extended_euclid(b % a, a);

    (y1 - (b / a) * x1, x1, gcd)
}
