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
    println!("speedy_part_2: {}", speedy_part_2(input));

    // assert_eq_same_input!(input, part1, speedy_part_1);
    assert_eq_same_input!(input, part2, speedy_part_2);

    simple_benchmark!(part1, input, 100);
    // simple_benchmark!(speedy_part_1, input);
    simple_benchmark!(part2, input, 100);
    simple_benchmark!(speedy_part_2, input, 10000);
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

const LINE_WIDTH: usize = {
    let mut line_width = 0;
    while include_str!("../input.txt").as_bytes()[line_width] != b'\n' {
        line_width += 1;
    }
    line_width
};

const LINES: usize = include_str!("../input.txt").as_bytes().len() / LINE_WIDTH;

fn speedy_part_2(input: &str) -> usize {
    let input = input.as_bytes();

    let mut mat = [[0; LINE_WIDTH]; LINES];

    let mut linei: usize = 0;
    let mut i: usize = 0;
    let mut id: usize = 1;

    for &c in input.iter() {
        match c {
            b'\n' => {
                linei += 1;
                i = 0;
            }
            b'*' => {
                mat[linei][i] = id;
                mat[linei][i + 1] = id;
                mat[linei][i.saturating_sub(1)] = id;
                mat[linei + 1][i] = id;
                mat[linei + 1][i + 1] = id;
                mat[linei + 1][i.saturating_sub(1)] = id;
                mat[linei.saturating_sub(1)][i] = id;
                mat[linei.saturating_sub(1)][i + 1] = id;
                mat[linei.saturating_sub(1)][i.saturating_sub(1)] = id;

                i += 1;
                id += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    let mut stars = Vec::with_capacity(id);
    stars.push(None);
    for _ in 1..id {
        stars.push(Some([0, 0]));
    }

    linei = 0;
    i = 0;

    let mut iter = &input[..];

    while let [c, ..] = iter {
        match c {
            b'\n' => {
                linei += 1;
                i = 0;
            }
            num @ b'0'..=b'9' => {
                let mut val = (num - b'0') as usize;
                let mut width = 1;
                let mut number_iter = &iter[1..];

                while let [c @ b'0'..=b'9', ..] = number_iter {
                    val = val * 10 + (c - b'0') as usize;
                    width += 1;
                    number_iter = &number_iter[1..];
                }

                (i..i + width).find(|ii| mat[linei][*ii] > 0).map(|ii| {
                    match stars[mat[linei][ii]] {
                        Some(ref mut star @ [0, 0]) => {
                            star[0] = val;
                        }
                        Some(ref mut star @ [_, 0]) => {
                            star[1] = val;
                        }
                        _ => unreachable!(),
                        // cannot happen, I could set it to None though.
                        // if star_overfilled { stars[mat[linei][ii]] = None; }
                    }
                });

                iter = &iter[width - 1..];
                i += width;
            }
            _ => {
                i += 1;
            }
        }

        iter = &iter[1..];
    }

    stars
        .into_iter()
        .filter_map(std::convert::identity)
        .filter(|star| star.len() == 2)
        .map(|star| star.iter().product::<usize>())
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

#[test]
fn test_speedy_part_2() {
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
    assert_eq!(speedy_part_2(test), 467835);
}
