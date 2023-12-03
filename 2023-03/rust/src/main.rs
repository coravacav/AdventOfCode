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
    simple_benchmark!(speedy_part_2, input);
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

#[cfg(not(debug_assertions))]
const LINE_WIDTH: usize = {
    let mut line_width = 0;
    while include_str!("../input.txt").as_bytes()[line_width] != b'\n' {
        line_width += 1;
    }
    line_width + 1
};

// when in release mode, we can use the const
#[cfg(not(debug_assertions))]
const INPUT_SIZE: usize = include_str!("../input.txt").len();

#[cfg(debug_assertions)]
const LINE_WIDTH: usize = 11;

#[cfg(debug_assertions)]
const INPUT_SIZE: usize = 110;

fn speedy_part_2(input: &str) -> usize {
    input
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(i, c)| {
            if i < LINE_WIDTH + 1
                || i >= INPUT_SIZE - LINE_WIDTH - 1
                || i % LINE_WIDTH == 0
                || i % LINE_WIDTH == LINE_WIDTH - 1
            {
                return 0;
            }

            match c {
                b'*' => handle_star(input.as_bytes(), i),
                _ => 0,
            }
        })
        .sum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Search {
    None,
    TopLeftUnbound,
    TopLeftPartial,
    TopMiddle,
    TopFull,
    TopRightPartial,
    TopRightUnbound,
    RightUnbound,
    BottomRightUnbound,
    BottomRightPartial,
    BottomMiddle,
    BottomFull,
    BottomLeftPartial,
    BottomLeftUnbound,
    LeftUnbound,
}

fn handle_star(input: &[u8], i: usize) -> usize {
    let mut first = Search::None;
    let mut second = Search::None;

    macro_rules! assign_or_return_0 {
        ($first:ident, $second:ident, $val:expr) => {
            if matches!($first, Search::None) {
                $first = $val;
            } else if matches!($second, Search::None) {
                $second = $val;
            } else {
                return 0;
            }
        };
    }

    /// walk_left
    fn w_l(input: &[u8], start: usize) -> usize {
        let mut i = start;
        while matches!(input[i], b'0'..=b'9') {
            if i == 0 {
                return 0;
            }
            i -= 1;
        }
        i + 1
    }

    /// walk_right
    fn w_r(input: &[u8], start: usize) -> usize {
        let mut i = start;
        while matches!(input[i], b'0'..=b'9') {
            if i == INPUT_SIZE - 2 {
                break;
            }
            i += 1;
        }
        i
    }

    fn read_precise(input: &[u8], start: usize, end: usize) -> usize {
        let mut number_iter = &input[start..=end];
        let mut val = 0;

        // Walk enough left to be sure we're at the start of the number

        while let [c @ b'0'..=b'9', ..] = number_iter {
            val = val * 10 + (c - b'0') as usize;
            number_iter = &number_iter[1..];
        }

        val
    }

    fn parse(input: &[u8], i: usize, search_location: Search) -> usize {
        match search_location {
            Search::None => 0,
            Search::TopLeftUnbound => {
                read_precise(input, w_l(input, i - LINE_WIDTH - 1), i - LINE_WIDTH + 1)
            }
            Search::TopLeftPartial => {
                read_precise(input, w_l(input, i - LINE_WIDTH - 1), i - LINE_WIDTH)
            }
            Search::TopMiddle => read_precise(input, i - LINE_WIDTH, i - LINE_WIDTH),
            Search::TopFull => read_precise(
                input,
                w_l(input, i - LINE_WIDTH - 1),
                w_r(input, i - LINE_WIDTH + 1),
            ),
            Search::TopRightPartial => {
                read_precise(input, i - LINE_WIDTH, w_r(input, i - LINE_WIDTH + 1))
            }
            Search::TopRightUnbound => {
                read_precise(input, i - LINE_WIDTH + 1, w_r(input, i - LINE_WIDTH + 1))
            }
            Search::RightUnbound => read_precise(input, i + 1, w_r(input, i + 1)),
            Search::BottomRightUnbound => {
                read_precise(input, i + LINE_WIDTH + 1, w_r(input, i + LINE_WIDTH + 1))
            }
            Search::BottomRightPartial => {
                read_precise(input, i + LINE_WIDTH, w_r(input, i + LINE_WIDTH + 1))
            }
            Search::BottomMiddle => read_precise(input, i + LINE_WIDTH, i + LINE_WIDTH),
            Search::BottomFull => read_precise(
                input,
                w_l(input, i + LINE_WIDTH - 1),
                w_r(input, i + LINE_WIDTH + 1),
            ),
            Search::BottomLeftPartial => {
                read_precise(input, w_l(input, i + LINE_WIDTH - 1), i + LINE_WIDTH)
            }
            Search::BottomLeftUnbound => {
                read_precise(input, w_l(input, i + LINE_WIDTH - 1), i + LINE_WIDTH - 1)
            }
            Search::LeftUnbound => read_precise(input, w_l(input, i - 1), i - 1),
        }
    }

    match [
        input[i - LINE_WIDTH - 1],
        input[i - LINE_WIDTH],
        input[i - LINE_WIDTH + 1],
    ] {
        [b'0'..=b'9', b'0'..=b'9', b'0'..=b'9'] => {
            assign_or_return_0!(first, second, Search::TopFull);
        }
        [b'0'..=b'9', _, b'0'..=b'9'] => {
            assign_or_return_0!(first, second, Search::TopLeftUnbound);
            assign_or_return_0!(first, second, Search::TopRightUnbound);
        }
        [b'0'..=b'9', b'0'..=b'9', _] => {
            assign_or_return_0!(first, second, Search::TopLeftPartial);
        }
        [_, b'0'..=b'9', b'0'..=b'9'] => {
            assign_or_return_0!(first, second, Search::TopRightPartial);
        }
        [b'0'..=b'9', _, _] => {
            assign_or_return_0!(first, second, Search::TopLeftUnbound);
        }
        [_, b'0'..=b'9', _] => {
            assign_or_return_0!(first, second, Search::TopMiddle);
        }
        [_, _, b'0'..=b'9'] => {
            assign_or_return_0!(first, second, Search::TopRightUnbound);
        }
        _ => {}
    }

    match input[i - 1] {
        b'0'..=b'9' => {
            assign_or_return_0!(first, second, Search::LeftUnbound);
        }
        _ => {}
    }

    match input[i + 1] {
        b'0'..=b'9' => {
            assign_or_return_0!(first, second, Search::RightUnbound);
        }
        _ => {}
    }

    match [
        input[i + LINE_WIDTH - 1],
        input[i + LINE_WIDTH],
        input[i + LINE_WIDTH + 1],
    ] {
        [b'0'..=b'9', b'0'..=b'9', b'0'..=b'9'] => {
            assign_or_return_0!(first, second, Search::BottomFull);
        }
        [b'0'..=b'9', _, b'0'..=b'9'] => {
            assign_or_return_0!(first, second, Search::BottomLeftUnbound);
            assign_or_return_0!(first, second, Search::BottomRightUnbound);
        }
        [b'0'..=b'9', b'0'..=b'9', _] => {
            assign_or_return_0!(first, second, Search::BottomLeftPartial);
        }
        [_, b'0'..=b'9', b'0'..=b'9'] => {
            assign_or_return_0!(first, second, Search::BottomRightPartial);
        }
        [b'0'..=b'9', _, _] => {
            assign_or_return_0!(first, second, Search::BottomLeftUnbound);
        }
        [_, b'0'..=b'9', _] => {
            assign_or_return_0!(first, second, Search::BottomMiddle);
        }
        [_, _, b'0'..=b'9'] => {
            assign_or_return_0!(first, second, Search::BottomRightUnbound);
        }
        _ => {}
    }

    if first == Search::None || second == Search::None {
        return 0;
    }

    parse(input, i, first) * parse(input, i, second)
}

#[test]
fn test_part1() {
    let test = r#"
467..114..
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
    let test = r#"
467..114..
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
...$...*..
.664...598"#;
    assert_eq!(speedy_part_2(test), 467835);
}
