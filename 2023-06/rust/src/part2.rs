use itertools::Itertools;
use rust_aoc_lib::part2;

use crate::quadratic_formula;

#[part2]
fn brute_force(input: &str) -> usize {
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
fn using_quadratic_formula(input: &str) -> usize {
    let mut input = input.as_bytes().split(|&b| b == b'\n').map(|input| {
        input
            .iter()
            .filter(|b| b.is_ascii_digit())
            .map(|b| (b - b'0') as usize)
            .fold(0, |acc, new| acc * 10 + new) as f64
    });

    let (first, last) = quadratic_formula(input.next().unwrap(), input.next().unwrap());

    last - first + 1
}
