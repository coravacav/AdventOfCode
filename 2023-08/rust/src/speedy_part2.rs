use std::sync::OnceLock;

use itertools::Itertools;
use rayon::prelude::*;
use rust_aoc_lib::{init, part2};

use crate::Instruction;

const fn convert_byte_to_index(c: u8) -> u8 {
    match c {
        b'A' => 0,
        b'B' => 1,
        b'C' => 2,
        b'D' => 3,
        b'E' => 4,
        b'F' => 5,
        b'G' => 6,
        b'H' => 7,
        b'I' => 8,
        b'J' => 9,
        b'K' => 10,
        b'L' => 11,
        b'M' => 12,
        b'N' => 13,
        b'O' => 14,
        b'P' => 15,
        b'Q' => 16,
        b'R' => 17,
        b'S' => 18,
        b'T' => 19,
        b'U' => 20,
        b'V' => 21,
        b'W' => 22,
        b'X' => 23,
        b'Y' => 24,
        b'Z' => 25,
        b'0' => 26,
        b'1' => 27,
        b'2' => 28,
        b'3' => 29,
        b'4' => 30,
        b'5' => 31,
        b'6' => 32,
        b'7' => 33,
        b'8' => 34,
        b'9' => 35,
        _ => unreachable!(),
    }
}

/// Rust doesn't let me const this, otherwise I would!
fn get_starting_locations() -> &'static Vec<u16> {
    static STARTING_LOCATIONS: OnceLock<&'static Vec<u16>> = OnceLock::new();
    STARTING_LOCATIONS.get_or_init(|| {
        Box::leak(Box::new(
            (0..35 * 35 * 35)
                .filter(|&i| i % 35 == convert_byte_to_index(b'A') as u16)
                .collect_vec(),
        ))
    })
}

#[init]
fn init() {
    get_starting_locations();
}

#[part2]
pub fn speedy_part2(input: &str) -> usize {
    let mut steps = Vec::new();
    let mut input = input.as_bytes().iter().peekable();

    let mut map = [(0, 0); 35 * 35 * 35];

    while let Some(c @ (b'L' | b'R')) = input.next() {
        match c {
            b'L' => steps.push(Instruction::Left),
            b'R' => steps.push(Instruction::Right),
            _ => unreachable!(),
        }
    }

    input.next(); // Skip the extra newline

    while input.peek().is_some() {
        let start = convert_byte_to_index(*input.next().unwrap()) as u16 * 35 * 35
            + convert_byte_to_index(*input.next().unwrap()) as u16 * 35
            + convert_byte_to_index(*input.next().unwrap()) as u16;

        input.next(); // ' '
        input.next(); // '='
        input.next(); // ' '
        input.next(); // '('

        let left = convert_byte_to_index(*input.next().unwrap()) as u16 * 35 * 35
            + convert_byte_to_index(*input.next().unwrap()) as u16 * 35
            + convert_byte_to_index(*input.next().unwrap()) as u16;

        input.next(); // ','
        input.next(); // ' '

        let right = convert_byte_to_index(*input.next().unwrap()) as u16 * 35 * 35
            + convert_byte_to_index(*input.next().unwrap()) as u16 * 35
            + convert_byte_to_index(*input.next().unwrap()) as u16;

        input.next(); // ')'
        input.next(); // '\n'

        map[start as usize] = (left, right);
    }

    let mut allowed_steps = steps.iter().cycle();

    get_starting_locations()
        .iter()
        .filter(|&&i| map[i as usize] != (0, 0))
        .map(|&key| {
            let mut steps = 0;
            let mut current = key;

            while current % 35 != convert_byte_to_index(b'Z') as u16 {
                let (left, right) = map[current as usize];

                steps += 1;

                current = match allowed_steps.next().unwrap() {
                    Instruction::Left => left,
                    Instruction::Right => right,
                };
            }

            steps
        })
        .reduce(lcm)
        .unwrap()
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}
