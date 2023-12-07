pub mod part1;
pub mod part2;

rust_aoc_lib::setup_distributed!();

pub fn quadratic_formula(time: f64, distance: f64) -> (usize, usize) {
    let a = -1.0;
    let b = time;
    let c = -(distance + 1.0f64);

    let discriminant = b.powi(2) - 4.0 * a * c;

    let sqrt_discriminant = discriminant.sqrt();
    let two_a = 2.0 * a;

    let upper_bound = (-b - sqrt_discriminant) / two_a;
    let lower_bound = (-b + sqrt_discriminant) / two_a;

    let upper_bound = upper_bound.floor() as usize;
    let lower_bound = lower_bound.ceil() as usize;

    (lower_bound, upper_bound)
}
