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
    // assert_eq_same_input!(input, part2, speedy_part_2);

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
    let mut val: Option<Rc<Cell<usize>>> = None;
    let mut number_vec: Vec<Option<Rc<Cell<usize>>>> = vec![None; INPUT_SIZE];

    let _ = input.as_bytes().iter().enumerate().map(|(i, c)| match c {
        b'0'..=b'9' => {
            if let Some(val) = &val {
                val.replace(val.get() * 10 + (c - b'0') as usize);
            } else {
                val = Some(Rc::new(Cell::new((c - b'0') as usize)));
            }

            number_vec[i] = Some(Rc::clone(val.as_ref().unwrap()));
        }
        _ => {
            val = None;
        }
    });

    input
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|&(i, _)| {
            i > LINE_WIDTH + 1
                && i < INPUT_SIZE - LINE_WIDTH - 1
                && i % LINE_WIDTH != 0
                && i % LINE_WIDTH != LINE_WIDTH - 1
        })
        .map(|(i, c)| match c {
            b'*' => {
                let mut found: [Option<&Rc<Cell<usize>>>; 2] = [None, None];

                for side in [
                    &number_vec[i - LINE_WIDTH - 1],
                    &number_vec[i - LINE_WIDTH],
                    &number_vec[i - LINE_WIDTH + 1],
                    &number_vec[i - 1],
                    &number_vec[i + 1],
                    &number_vec[i + LINE_WIDTH - 1],
                    &number_vec[i + LINE_WIDTH],
                    &number_vec[i + LINE_WIDTH + 1],
                ] {
                    let side = match side {
                        Some(side) => side,
                        None => continue,
                    };

                    match found {
                        [None, None] => {
                            found[0] = Some(side);
                        }
                        [Some(c), None] if c != side => {
                            found[1] = Some(side);
                        }
                        [Some(c1), Some(c2)] if c1 != side && c2 != side && c1 != c2 => {
                            return 0;
                        }
                        _ => {}
                    }
                }

                found[0].map(|f| f.get()).unwrap_or(0) * found[1].map(|f| f.get()).unwrap_or(0)
            }
            _ => 0,
        })
        .sum()
}

// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// enum Search {
//     TopLeftUnbound,
//     TopLeftPartial,
//     TopMiddle,
//     TopFull,
//     TopRightPartial,
//     TopRightUnbound,
//     RightUnbound,
//     BottomRightUnbound,
//     BottomRightPartial,
//     BottomMiddle,
//     BottomFull,
//     BottomLeftPartial,
//     BottomLeftUnbound,
//     LeftUnbound,
// }

// fn handle_star(input: &[u8], i: usize) -> usize {
//     let mut first = None;
//     let mut second = None;

//     macro_rules! assign_or_return_0 {
//         ($first:ident, $second:ident, $val:expr) => {
//             if matches!($first, None) {
//                 $first = Some($val);
//             } else if matches!($second, None) {
//                 $second = Some($val);
//             } else {
//                 return 0;
//             }
//         };
//     }

//     fn read_precise(input: &[u8], start: usize, end: usize) -> usize {
//         match &input[start..=end] {
//             [h @ b'0'..=b'9', t @ b'0'..=b'9', o @ b'0'..=b'9'] => {
//                 (h - b'0') as usize * 100 + (t - b'0') as usize * 10 + (o - b'0') as usize
//             }
//             [t @ b'0'..=b'9', o @ b'0'..=b'9']
//             | [_, t @ b'0'..=b'9', o @ b'0'..=b'9']
//             | [t @ b'0'..=b'9', o @ b'0'..=b'9', _] => {
//                 (t - b'0') as usize * 10 + (o - b'0') as usize
//             }
//             [_, _, o @ b'0'..=b'9']
//             | [_, o @ b'0'..=b'9', _]
//             | [o @ b'0'..=b'9', _, _]
//             | [_, o @ b'0'..=b'9']
//             | [o @ b'0'..=b'9', _]
//             | [o @ b'0'..=b'9'] => (o - b'0') as usize,
//             _ => unreachable!(),
//         }
//     }

//     fn parse(input: &[u8], i: usize, search_location: Search) -> usize {
//         let (left, right) = match search_location {
//             Search::BottomFull => (i + LINE_WIDTH - 1, i + LINE_WIDTH + 1),
//             Search::BottomLeftPartial => (i + LINE_WIDTH - 2, i + LINE_WIDTH),
//             Search::BottomLeftUnbound => (i + LINE_WIDTH - 3, i + LINE_WIDTH - 1),
//             Search::BottomMiddle => (i + LINE_WIDTH, i + LINE_WIDTH),
//             Search::BottomRightPartial => (i + LINE_WIDTH, i + LINE_WIDTH + 2),
//             Search::BottomRightUnbound => (i + LINE_WIDTH + 1, i + LINE_WIDTH + 3),
//             Search::LeftUnbound => (i - 3, i - 1),
//             Search::RightUnbound => (i + 1, i + 3),
//             Search::TopFull => (i - LINE_WIDTH - 1, i - LINE_WIDTH + 1),
//             Search::TopLeftPartial => (i - LINE_WIDTH - 2, i - LINE_WIDTH),
//             Search::TopLeftUnbound => (i - LINE_WIDTH - 3, i - LINE_WIDTH - 1),
//             Search::TopMiddle => (i - LINE_WIDTH, i - LINE_WIDTH),
//             Search::TopRightPartial => (i - LINE_WIDTH, i - LINE_WIDTH + 2),
//             Search::TopRightUnbound => (i - LINE_WIDTH + 1, i - LINE_WIDTH + 3),
//         };

//         read_precise(input, left, right)
//     }

//     match [
//         input[i - LINE_WIDTH - 1],
//         input[i - LINE_WIDTH],
//         input[i - LINE_WIDTH + 1],
//     ] {
//         [b'0'..=b'9', b'0'..=b'9', b'0'..=b'9'] => {
//             assign_or_return_0!(first, second, Search::TopFull);
//         }
//         [b'0'..=b'9', _, b'0'..=b'9'] => {
//             assign_or_return_0!(first, second, Search::TopLeftUnbound);
//             assign_or_return_0!(first, second, Search::TopRightUnbound);
//         }
//         [b'0'..=b'9', b'0'..=b'9', _] => {
//             assign_or_return_0!(first, second, Search::TopLeftPartial);
//         }
//         [_, b'0'..=b'9', b'0'..=b'9'] => {
//             assign_or_return_0!(first, second, Search::TopRightPartial);
//         }
//         [b'0'..=b'9', _, _] => {
//             assign_or_return_0!(first, second, Search::TopLeftUnbound);
//         }
//         [_, b'0'..=b'9', _] => {
//             assign_or_return_0!(first, second, Search::TopMiddle);
//         }
//         [_, _, b'0'..=b'9'] => {
//             assign_or_return_0!(first, second, Search::TopRightUnbound);
//         }
//         _ => {}
//     }

//     match input[i - 1] {
//         b'0'..=b'9' => {
//             assign_or_return_0!(first, second, Search::LeftUnbound);
//         }
//         _ => {}
//     }

//     match input[i + 1] {
//         b'0'..=b'9' => {
//             assign_or_return_0!(first, second, Search::RightUnbound);
//         }
//         _ => {}
//     }

//     match [
//         input[i + LINE_WIDTH - 1],
//         input[i + LINE_WIDTH],
//         input[i + LINE_WIDTH + 1],
//     ] {
//         [b'0'..=b'9', b'0'..=b'9', b'0'..=b'9'] => {
//             assign_or_return_0!(first, second, Search::BottomFull);
//         }
//         [b'0'..=b'9', _, b'0'..=b'9'] => {
//             assign_or_return_0!(first, second, Search::BottomLeftUnbound);
//             assign_or_return_0!(first, second, Search::BottomRightUnbound);
//         }
//         [b'0'..=b'9', b'0'..=b'9', _] => {
//             assign_or_return_0!(first, second, Search::BottomLeftPartial);
//         }
//         [_, b'0'..=b'9', b'0'..=b'9'] => {
//             assign_or_return_0!(first, second, Search::BottomRightPartial);
//         }
//         [b'0'..=b'9', _, _] => {
//             assign_or_return_0!(first, second, Search::BottomLeftUnbound);
//         }
//         [_, b'0'..=b'9', _] => {
//             assign_or_return_0!(first, second, Search::BottomMiddle);
//         }
//         [_, _, b'0'..=b'9'] => {
//             assign_or_return_0!(first, second, Search::BottomRightUnbound);
//         }
//         _ => {}
//     }

//     if first == None || second == None {
//         return 0;
//     }

//     parse(input, i, first.unwrap()) * parse(input, i, second.unwrap())
// }

// This was the old version, marginally slower.
// fn speedy_part_2(input: &str) -> usize {
//     let mut mat = [[0u16; LINE_WIDTH]; LINES];
//     let mut line = 0;
//     let mut col = 0;

//     let mut star_locations = Vec::new();
//     let mut number_values = Vec::new();
//     number_values.push(0);
//     number_values.push(0);
//     let mut val = 0;

//     for c in input.as_bytes() {
//         match c {
//             b'*' => {
//                 mat[line][col] = 1;
//                 star_locations.push((line, col));
//                 col += 1;
//             }
//             c @ b'0'..=b'9' => {
//                 incr_num!(val, c);
//                 mat[line][col] = number_values.len() as u16;
//                 col += 1;
//                 continue;
//             }
//             b'\n' => {
//                 line += 1;
//                 col = 0;
//             }
//             _ => {
//                 col += 1;
//             }
//         };

//         if val > 0 {
//             number_values.push(val);
//             val = 0;
//         }
//     }

//     star_locations
//         .into_iter()
//         .filter_map(|(x, y)| {
//             let mut seen = [0; 2];

//             for (i, j) in [
//                 (x, y - 1),
//                 (x, y + 1),
//                 (x - 1, y - 1),
//                 (x - 1, y),
//                 (x - 1, y + 1),
//                 (x + 1, y - 1),
//                 (x + 1, y),
//                 (x + 1, y + 1),
//             ] {
//                 if mat[i][j] > 1 {
//                     match seen {
//                         [a, _] | [_, a] if a == mat[i][j] => {}
//                         [0, _] => {
//                             seen[0] = mat[i][j];
//                         }
//                         [_, 0] => {
//                             seen[1] = mat[i][j];
//                             break; // Technically should keep checking but all inputs I've found allow this.
//                         }
//                         _ => return None,
//                     }
//                 }
//             }

//             Some(number_values[seen[0] as usize] * number_values[seen[1] as usize])
//         })
//         .sum()
// }

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
    let test = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$...*..
.664...598"#;
    assert_eq!(speedy_part_2(test.trim()), 467835);
}
