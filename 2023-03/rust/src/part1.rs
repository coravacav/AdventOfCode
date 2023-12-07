use std::{cell::Cell, rc::Rc};

use itertools::Itertools;
use rust_aoc_lib::part1;

#[part1]
fn part1(input: &str) -> usize {
    let mut all_nums_and_locations = Vec::new();
    for (linei, line) in input.lines().enumerate() {
        let mut val = Rc::new(Cell::new(0));
        for (i, c) in line.chars().enumerate() {
            if c == '.' {
                val = Rc::new(Cell::new(0));
                continue;
            }
            if c.is_numeric() {
                all_nums_and_locations.push((linei, i, Rc::clone(&val)));

                val.replace(val.get() * 10 + (c as usize - '0' as usize));
                continue;
            }
            val = Rc::new(Cell::new(0));
        }
    }

    let mut all_valid_locations = Vec::new();

    for (linei, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c == '.' || c.is_numeric() {
                continue;
            }

            all_valid_locations.push((linei + 1, i));
            all_valid_locations.push((linei, i));
            all_valid_locations.push((linei.saturating_sub(1), i));
            all_valid_locations.push((linei + 1, i + 1));
            all_valid_locations.push((linei, i + 1));
            all_valid_locations.push((linei.saturating_sub(1), i + 1));
            all_valid_locations.push((linei + 1, i.saturating_sub(1)));
            all_valid_locations.push((linei, i.saturating_sub(1)));
            all_valid_locations.push((linei.saturating_sub(1), i.saturating_sub(1)));
        }
    }

    all_nums_and_locations
        .into_iter()
        .filter(|(linei, i, _)| {
            all_valid_locations
                .iter()
                .any(|(linei2, i2)| linei == linei2 && i == i2)
        })
        .unique_by(|(_, _, val)| Rc::as_ptr(val) as usize)
        .map(|(_, _, val)| val.get())
        .sum()
}
