use std::collections::HashMap;

use itertools::Itertools;
use rust_aoc_lib::part1;

fn parse(row: &str) -> (Vec<char>, Vec<u8>) {
    let (cell_str, num_str) = row.split_once(' ').unwrap();

    let numbers = num_str
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect_vec();

    (cell_str.chars().collect_vec(), numbers)
}

fn generate_all_possibilities<'a>(
    cache: &mut HashMap<(&'a [char], &'a [u8], u8), usize>,
    cells: &'a [char],
    mut numbers: &'a [u8],
    mut number_of_damaged: u8,
) -> usize {
    if let Some(&val) = cache.get(&(cells, numbers, number_of_damaged)) {
        return val;
    }

    if numbers.is_empty() {
        return 0;
    }

    if Some(&number_of_damaged) == numbers.first() {
        numbers = &numbers[1..];
        number_of_damaged = u8::MAX;

        if numbers.is_empty() {
            return if cells.contains(&'#') { 0 } else { 1 };
        }
    }

    let Some(cell) = cells.first() else {
        return 0;
    };

    fn do_damaged<'a>(
        cache: &mut HashMap<(&'a [char], &'a [u8], u8), usize>,
        cells: &'a [char],
        numbers: &'a [u8],
        number_of_damaged: u8,
    ) -> usize {
        if number_of_damaged == u8::MAX {
            0
        } else {
            let res =
                generate_all_possibilities(cache, &cells[1..], numbers, number_of_damaged + 1);
            cache.insert((&cells[1..], numbers, number_of_damaged + 1), res);
            res
        }
    }

    fn do_operational<'a>(
        cache: &mut HashMap<(&'a [char], &'a [u8], u8), usize>,
        cells: &'a [char],
        numbers: &'a [u8],
        number_of_damaged: u8,
    ) -> usize {
        if number_of_damaged > 0 && number_of_damaged != u8::MAX {
            0
        } else {
            let res = generate_all_possibilities(cache, &cells[1..], numbers, 0);
            cache.insert((&cells[1..], numbers, 0), res);
            res
        }
    }

    match cell {
        '#' => do_damaged(cache, cells, numbers, number_of_damaged),
        '?' => {
            do_damaged(cache, cells, numbers, number_of_damaged)
                + do_operational(cache, cells, numbers, number_of_damaged)
        }
        '.' => do_operational(cache, cells, numbers, number_of_damaged),
        _ => unreachable!(),
    }
}

#[part1]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(parse)
        .map(|(cells, numbers)| {
            generate_all_possibilities(&mut HashMap::new(), &cells, &numbers, 0)
        })
        .sum()
}
