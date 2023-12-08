use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use itertools::Itertools;
use rust_aoc_lib::part2;
use tap::Tap;

#[part2]
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

// Removed some solutions that were buggy and I didn't care for.
