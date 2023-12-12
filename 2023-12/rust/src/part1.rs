use std::collections::VecDeque;

use bit_vec::BitVec;
use itertools::Itertools;
use rust_aoc_lib::part1;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Unknown(u8),
    Operational(u8),
    Damaged(u8),
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unknown(n) => write!(f, "{}", "?".repeat((*n) as usize)),
            Operational(n) => write!(f, "{}", ".".repeat((*n) as usize)),
            Damaged(n) => write!(f, "{}", "#".repeat((*n) as usize)),
        }
    }
}

use Cell::*;

fn parse(row: &str) -> (VecDeque<Cell>, VecDeque<u8>) {
    let mut cells = VecDeque::new();

    let (cell_str, num_str) = row.split_once(' ').unwrap();

    let numbers = num_str
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<VecDeque<_>>();

    let mut cell_chars = cell_str.chars().peekable();

    macro_rules! loop_till_other {
        ($c:expr) => {{
            let mut count = 1;

            while let Some($c) = cell_chars.peek() {
                count += 1;
                cell_chars.next();
            }

            count
        }};
    }

    while cell_chars.peek().is_some() {
        cells.push_back(match cell_chars.next().unwrap() {
            '#' => Damaged(loop_till_other!('#')),
            '.' => Operational(loop_till_other!('.')),
            '?' => Unknown(loop_till_other!('?')),
            _ => unreachable!(),
        });
    }

    (cells, numbers)
}

fn ensure_proper_spacing(
    (mut cells, numbers): (VecDeque<Cell>, VecDeque<u8>),
) -> (VecDeque<Cell>, VecDeque<u8>) {
    let mut new_cells = VecDeque::new();

    for i in 0..cells.len() {
        new_cells.push_back(cells[i]);

        if let Damaged(_) = cells[i] {
            if i != cells.len() - 1 {
                if let Unknown(m) = cells[i + 1] {
                    cells.push_back(Operational(1));
                    cells.push_back(Unknown(m - 1));
                }
            }
        }
    }

    let mut cells = new_cells;
    let mut new_cells = VecDeque::new();

    for i in 0..cells.len() {
        new_cells.push_back(cells[i]);

        if let Damaged(_) = cells[i] {
            if i != 0 {
                if let Unknown(m) = cells[i - 1] {
                    cells.push_back(Unknown(m - 1));
                    cells.push_back(Operational(1));
                }
            }
        }
    }

    let cells = new_cells
        .into_iter()
        .filter(|c| !matches!(c, Unknown(0)))
        .filter(|c| matches!(c, Damaged(_) | Unknown(_)))
        .collect::<VecDeque<_>>();

    (cells, numbers)
}

fn drop_safe(
    (mut cells, mut numbers): (VecDeque<Cell>, VecDeque<u8>),
) -> (VecDeque<Cell>, VecDeque<u8>) {
    loop {
        if numbers.is_empty() {
            break;
        }

        if let Some(Damaged(c)) = cells.front() {
            if numbers.front().unwrap() == c {
                numbers.pop_front();
                cells.pop_front();
                continue;
            }
        }

        if let Some(Damaged(c)) = cells.back() {
            if numbers.back().unwrap() == c {
                numbers.pop_back();
                cells.pop_back();
                continue;
            }
        }

        break;
    }

    (cells, numbers)
}

fn reintroduce_operational(
    (cells, numbers): (VecDeque<Cell>, VecDeque<u8>),
) -> (VecDeque<Cell>, VecDeque<u8>) {
    let mut new_cells = VecDeque::new();

    // If there are two adjacent "unknown", put an operational single dot in the middle
    for i in 0..cells.len() {
        new_cells.push_back(cells[i]);

        if let Unknown(_) = cells[i] {
            if let Some(Unknown(_)) = cells.get(i + 1) {
                new_cells.push_back(Operational(1));
            }
        }
    }

    (new_cells, numbers)
}

fn generate_all_possibilities(cells: VecDeque<Cell>) -> Vec<BitVec> {
    let mut possibilities: Vec<BitVec> = vec![BitVec::new()];

    for cell in cells {
        match cell {
            Damaged(n) => {
                possibilities
                    .iter_mut()
                    .for_each(|s| (0..n).for_each(|_| s.push(true)));
            }
            Unknown(n) => {
                for _ in 0..n {
                    possibilities = possibilities
                        .into_iter()
                        .flat_map(|s| {
                            let mut option1 = s.clone();
                            let mut option2 = s;
                            option1.push(false);
                            option2.push(true);
                            [option1, option2]
                        })
                        .collect_vec()
                }
            }
            Operational(_) => {
                possibilities.iter_mut().for_each(|s| s.push(false));
            }
        }
    }

    possibilities.into_iter().unique().collect()
}

fn starts_with(a: &BitVec, ai: usize, len: usize) -> bool {
    (ai..ai + len).all(|i| matches!(a.get(i), Some(true)))
}

fn is_valid(a: &BitVec, numbers: &VecDeque<u8>) -> bool {
    let mut numbers = numbers.iter().peekable();
    let mut ai = 0;

    while numbers.peek().is_some() && ai < a.len() {
        if starts_with(a, ai, **numbers.peek().unwrap() as usize) {
            ai += **numbers.peek().unwrap() as usize;
            numbers.next();

            if ai == a.len() && numbers.peek().is_none() {
                return true;
            }

            if ai < a.len() && a[ai] {
                return false;
            }
        }

        if ai == a.len() || matches!(a.get(ai), Some(true)) {
            return false;
        }

        ai += 1;
    }

    if numbers.peek().is_some() {
        return false;
    }

    for i in ai..a.len() {
        if a[i] {
            return false;
        }
    }

    true
}

#[part1]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(parse)
        // .map(ensure_proper_spacing)
        // .map(drop_safe)
        // .map(reintroduce_operational)
        // We could do more passes to drop more starting values, but that can be saved for benchmarking
        // .inspect(|(cells, numbers)| {
        //     for cell in cells.iter() {
        //         print!("{} ", cell);
        //     }
        //     for (i, num) in numbers.iter().enumerate() {
        //         print!("{}", num);
        //         if i != numbers.len() - 1 {
        //             print!(",");
        //         }
        //     }
        //     println!();
        // })
        .map(|(cells, numbers)| (generate_all_possibilities(cells), numbers))
        // .inspect(|(possibilities, _)| {
        //     for possibility in possibilities.iter() {
        //         println!("{}", possibility);
        //     }
        // })
        .map(|(possibilities, numbers)| {
            possibilities
                .iter()
                // .inspect(|s| println!("{}", pretty_print_bitvec(s)))
                .filter(|s| is_valid(s, &numbers))
                // .inspect(|s| println!("{} (valid)", pretty_print_bitvec(s)))
                .count()
        })
        // .inspect(|n| println!("{}", n))
        .sum()
}

fn pretty_print_bitvec(a: &BitVec) -> String {
    a.iter().map(|b| if b { '#' } else { '.' }).collect()
}
