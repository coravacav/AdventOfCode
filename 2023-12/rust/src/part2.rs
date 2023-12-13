use std::collections::HashMap;

use itertools::Itertools;
use rayon::prelude::*;
use rust_aoc_lib::part2;

fn parse(row: &&[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut row = row.split(|c| c == &b' ');

    (
        row.next().unwrap().to_owned(),
        std::str::from_utf8(row.next().unwrap())
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u8>().unwrap())
            .collect_vec(),
    )
}

fn generate_all_possibilities<'a>(
    cache: &mut HashMap<(&'a [u8], &'a [u8], u8), usize>,
    cells: &'a [u8],
    mut numbers: &'a [u8],
    mut number_of_damaged: u8,
) -> usize {
    if cache.contains_key(&(cells, numbers, number_of_damaged)) {
        return *cache.get(&(cells, numbers, number_of_damaged)).unwrap();
    }

    if numbers.is_empty() {
        return 0;
    }

    if Some(&number_of_damaged) == numbers.first() {
        numbers = &numbers[1..];
        number_of_damaged = u8::MAX;

        if numbers.is_empty() {
            return if cells.contains(&b'#') { 0 } else { 1 };
        }
    }

    let Some(cell) = cells.first() else {
        return 0;
    };

    fn do_damaged<'a>(
        cache: &mut HashMap<(&'a [u8], &'a [u8], u8), usize>,
        cells: &'a [u8],
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
        cache: &mut HashMap<(&'a [u8], &'a [u8], u8), usize>,
        cells: &'a [u8],
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
        b'#' => do_damaged(cache, cells, numbers, number_of_damaged),
        b'?' => {
            do_operational(cache, cells, numbers, number_of_damaged)
                + do_damaged(cache, cells, numbers, number_of_damaged)
        }
        b'.' => do_operational(cache, cells, numbers, number_of_damaged),
        _ => unreachable!(),
    }
}

#[part2]
pub fn part2(input: &str) -> usize {
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .collect_vec()
        .par_iter()
        .map(parse)
        .map(|(cells, numbers)| {
            let repeat_count = 5;
            let cells_len = cells.len() * repeat_count + repeat_count - 1;
            let numbers_len = numbers.len() * repeat_count;

            (
                cells
                    .into_iter()
                    .chain(vec![b'?'])
                    .cycle()
                    .take(cells_len)
                    .collect_vec(),
                numbers.into_iter().cycle().take(numbers_len).collect_vec(),
            )
        })
        .map(|(cells, numbers)| {
            generate_all_possibilities(&mut HashMap::new(), &cells, &numbers, 0)
        })
        .sum()
}
