use itertools::Itertools;
use rust_aoc_lib::{part2, read_number};

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
    let mut input = input.as_bytes();

    let time = {
        let mut val = 0;

        while let [c, rest @ ..] = input {
            match c {
                c @ b'0'..=b'9' => {
                    val = val * 10 + (c - b'0') as usize;
                }
                b'\n' => break,
                _ => {}
            }

            input = rest;
        }

        val as f64
    };

    let distance = {
        let mut val = 0;

        while let [c, rest @ ..] = input {
            match c {
                c @ b'0'..=b'9' => {
                    val = val * 10 + (c - b'0') as usize;
                }
                _ => {}
            }

            input = rest;
        }

        val as f64
    };

    let (first, last) = quadratic_formula(time, distance);

    last - first + 1
}
