use std::collections::BTreeMap;

use itertools::Itertools;
use rust_aoc_lib::part2;

use crate::Instruction;

#[part2]
pub fn part2(input: &str) -> usize {
    let mut steps = Vec::new();
    let mut lines = input.lines();

    let mut map = BTreeMap::new();

    for char in lines.next().unwrap().chars() {
        match char {
            'L' => steps.push(Instruction::Left),
            'R' => steps.push(Instruction::Right),
            _ => panic!("Invalid input"),
        }
    }

    let mut allowed_steps = steps.iter().cycle();

    lines.next();

    for line in lines {
        let (start, rest) = line.split_once(" = (").unwrap();
        let (left, rest) = rest.split_once(", ").unwrap();
        let (right, _) = rest.split_once(')').unwrap();

        let start = start.as_bytes();
        let start = [0, 0, 0, 0, 0, start[0], start[1], start[2]];

        let left = left.as_bytes();
        let left = [0, 0, 0, 0, 0, left[0], left[1], left[2]];

        let right = right.as_bytes();
        let right = [0, 0, 0, 0, 0, right[0], right[1], right[2]];

        map.insert(
            u64::from_be_bytes(start),
            (u64::from_be_bytes(left), u64::from_be_bytes(right)),
        );
    }

    let current = map
        .keys()
        .filter(|&key| key.to_be_bytes()[7] == b'A')
        .collect_vec();

    // for each initial key, find the first occurence of ends with Z

    let dists = current
        .iter()
        .map(|&key| {
            let mut steps = 0;
            let mut current = key;

            while current.to_be_bytes()[7] != b'Z' {
                let (left, right) = map.get(current).unwrap();

                steps += 1;

                current = match allowed_steps.next().unwrap() {
                    Instruction::Left => left,
                    Instruction::Right => right,
                };
            }

            steps
        })
        .collect_vec();

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

    dists.into_iter().reduce(lcm).unwrap()
}
