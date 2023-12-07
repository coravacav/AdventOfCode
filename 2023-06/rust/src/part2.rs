use itertools::Itertools;
use rust_aoc_lib::part2;

#[part2]
fn part2(input: &str) -> usize {
    let both = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .filter_map(Result::ok)
        .collect_vec();

    let (time, distance) = both.split_at(both.len() / 2);

    fn passes_distance(pressed_time: usize, total_time: usize, distance: usize) -> usize {
        let speed = pressed_time;
        let remaining_time = total_time - pressed_time;
        let covered_distance = speed * remaining_time;
        if covered_distance > distance {
            1
        } else {
            0
        }
    }

    let time = time
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let distance = distance
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    (0..time)
        .map(|pressed_time| passes_distance(pressed_time, time, distance))
        .sum::<usize>()
}

#[part2]
fn speedy_part_2(input: &str) -> usize {
    let mut input = input.as_bytes();

    let time = {
        let mut val = 0;

        while let [c, ..] = input {
            match c {
                c @ b'0'..=b'9' => {
                    val = val * 10 + (c - b'0') as usize;
                }
                b'\n' => break,
                _ => {}
            }

            input = &input[1..];
        }

        val as f64
    };

    let distance = {
        let mut val = 0;

        while let [c, ..] = input {
            match c {
                c @ b'0'..=b'9' => {
                    val = val * 10 + (c - b'0') as usize;
                }
                _ => {}
            }

            input = &input[1..];
        }

        val as f64
    };

    // quadratic formula

    let a = -1.0;
    let b = time;
    let c: f64 = -(distance + 1.0f64);

    let discriminant = b.powi(2) - 4.0 * a * c;

    let sqrt_discriminant = discriminant.sqrt();
    let two_a = 2.0 * a;

    let last = (-b - sqrt_discriminant) / two_a;
    let first = (-b + sqrt_discriminant) / two_a;

    let first = first.ceil() as usize;
    let last = last.floor() as usize;

    last - first + 1
}
