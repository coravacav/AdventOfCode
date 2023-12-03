use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use itertools::Itertools;
#[allow(unused_imports)]
use rust_aoc_lib::*;
use tap::Tap;

fn main() {
    let input = include_str!("../input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    // println!("speedy_part_1: {}", speedy_part_1(input));
    // println!("speedy_part_2: {}", speedy_part_2(input));

    // assert_eq_same_input!(input, part1, speedy_part_1);
    // assert_eq_same_input!(input, part2, speedy_part_2);

    // simple_benchmark!(part1, input);
    // simple_benchmark!(speedy_part_1, input);
    // simple_benchmark!(part2, input);
    // simple_benchmark!(speedy_part_2, input);
}

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

fn part2(input: &str) -> usize {
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

            let a = Rc::new(RefCell::new((Rc::new(RefCell::new(0)), c, linei, i)));

            all_valid_locations.push((linei + 1, i, Rc::clone(&a)));
            all_valid_locations.push((linei, i, Rc::clone(&a)));
            all_valid_locations.push((linei.saturating_sub(1), i, Rc::clone(&a)));
            all_valid_locations.push((linei + 1, i + 1, Rc::clone(&a)));
            all_valid_locations.push((linei, i + 1, Rc::clone(&a)));
            all_valid_locations.push((linei.saturating_sub(1), i + 1, Rc::clone(&a)));
            all_valid_locations.push((linei + 1, i.saturating_sub(1), Rc::clone(&a)));
            all_valid_locations.push((linei, i.saturating_sub(1), Rc::clone(&a)));
            all_valid_locations.push((linei.saturating_sub(1), i.saturating_sub(1), Rc::clone(&a)));
        }
    }

    all_nums_and_locations
        .into_iter()
        .filter_map(|(linei, i, val)| {
            if let Some((_, _, s)) = all_valid_locations
                .iter()
                .find(|(linei2, i2, _)| linei == *linei2 && i == *i2)
            {
                Some((linei, i, val, s))
            } else {
                None
            }
        })
        .map(|s| (s.2, s.3))
        .unique_by(|(val, _)| Rc::as_ptr(val) as usize)
        .filter(|(_, s)| s.borrow().1 == '*')
        .map(|s| s.tap(|s| *s.1.borrow_mut().0.borrow_mut() += 1))
        .sorted_by(|(_, s1), (_, s2)| (Rc::as_ptr(s1) as usize).cmp(&(Rc::as_ptr(s2) as usize)))
        .group_by(|(_, s)| Rc::as_ptr(s) as usize)
        .into_iter()
        .map(|(_, group)| {
            let group = group.collect_vec();

            if group.len() != 2 {
                return 0;
            }

            let one = group[0].0.get();
            let two = group[1].0.get();

            one * two
        })
        .sum()
}

#[test]
fn test_part1() {
    let test = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    assert_eq!(part1(test), 4361);
}

#[test]
fn test_part2() {
    let test = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    assert_eq!(part2(test), 467835);
}
