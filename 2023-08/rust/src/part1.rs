use std::collections::HashMap;

use rust_aoc_lib::part1;

use crate::Instruction;

#[part1]
pub fn part1(input: &str) -> usize {
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut steps = Vec::new();
    let mut lines = input.lines();

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

        map.insert(start, (left, right));
    }

    let mut current = "AAA";
    let mut steps = 0;

    while !matches!(current, "ZZZ") {
        let (left, right) = map.get(current).unwrap();

        steps += 1;

        match allowed_steps.next().unwrap() {
            Instruction::Left => current = left,
            Instruction::Right => current = right,
        }
    }

    steps
}
