// From @ehllie on discord.

use std::collections::HashMap;

use rust_aoc_lib::{part1, part2};

type Galaxies = Vec<((isize, isize), (isize, isize))>;

#[part1]
pub fn part_one(input: &str) -> usize {
    let galaxies = parse_universe(input);
    galaxy_distances(&galaxies, 2) as usize
}

#[part2]
pub fn part_two(input: &str) -> usize {
    let galaxies = parse_universe(input);
    galaxy_distances(&galaxies, 1_000_000) as usize
}

fn galaxy_distances(galaxies: &Galaxies, expansion_rate: isize) -> isize {
    let expansion_rate = expansion_rate - 1;
    galaxies.iter().enumerate().fold(0, |acc, (i, (g1, e1))| {
        galaxies.iter().skip(i + 1).fold(acc, |acc, (g2, e2)| {
            let e1 = (e1.0 * expansion_rate, e1.1 * expansion_rate);
            let e2 = (e2.0 * expansion_rate, e2.1 * expansion_rate);
            let (x1, y1) = (g1.0 + e1.0, g1.1 + e1.1);
            let (x2, y2) = (g2.0 + e2.0, g2.1 + e2.1);
            let manhatan = (x1 - x2).abs() + (y1 - y2).abs();
            acc + manhatan
        })
    })
}

fn parse_universe(input: &str) -> Galaxies {
    let mut columns = HashMap::<isize, Vec<(isize, isize, isize)>>::new();
    let mut expanded_rows = 0;
    let mut row_length = 0;
    for (row, line) in input.lines().enumerate() {
        let mut empty_row = true;
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                if let Some(column) = columns.get_mut(&(col as isize)) {
                    column.push((row as isize, 0, expanded_rows));
                } else {
                    columns.insert(col as isize, Vec::from([(row as isize, 0, expanded_rows)]));
                }
                empty_row = false;
            }
        }
        row_length = line.len() as isize;
        if empty_row {
            expanded_rows += 1;
        }
    }

    let mut expanded_columns = 0;
    for col in 0..row_length {
        if let Some(column) = columns.remove(&col) {
            columns.insert(
                col,
                column
                    .into_iter()
                    .map(|(row, _, e_row)| (row, expanded_columns, e_row))
                    .collect(),
            );
        } else {
            // println!("Column {} is empty", col);
            expanded_columns += 1;
        }
    }

    columns
        .into_iter()
        .flat_map(|(col, rows)| {
            rows.into_iter()
                .map(move |(row, e_col, e_row)| ((col, row), (e_col, e_row)))
        })
        .collect()
}

type GalaxiesFast = Vec<(isize, isize)>;

#[part1]
pub fn part_one_fast(input: &str) -> usize {
    let galaxies = parse_universe_fast(input, 2);
    galaxy_distances_fast(&galaxies) as usize
}

#[part2]
pub fn part_two_fast(input: &str) -> usize {
    let galaxies = parse_universe_fast(input, 1_000_000);
    galaxy_distances_fast(&galaxies) as usize
}

fn galaxy_distances_fast(galaxies: &GalaxiesFast) -> isize {
    galaxies.iter().enumerate().fold(0, |acc, (i, (x1, y1))| {
        galaxies.iter().skip(i + 1).fold(acc, |acc, (x2, y2)| {
            let manhatan = (x1 - x2).abs() + (y1 - y2).abs();
            acc + manhatan
        })
    })
}

fn parse_universe_fast(input: &str, expansion_rate: isize) -> GalaxiesFast {
    let mut columns = HashMap::<isize, Vec<(isize, isize, isize)>>::new();
    let mut expanded_rows = 0;
    let mut row_length = 0;
    for (row, line) in input.lines().enumerate() {
        let mut empty_row = true;
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                if let Some(column) = columns.get_mut(&(col as isize)) {
                    column.push((row as isize, 0, expanded_rows));
                } else {
                    columns.insert(col as isize, Vec::from([(row as isize, 0, expanded_rows)]));
                }
                empty_row = false;
            }
        }
        row_length = line.len() as isize;
        if empty_row {
            expanded_rows += 1;
        }
    }

    let mut expanded_columns = 0;
    for col in 0..row_length {
        if let Some(column) = columns.get_mut(&col) {
            for row in column.iter_mut() {
                *row = (row.0, expanded_columns, row.2);
            }
        } else {
            expanded_columns += 1;
        }
    }

    let expansion_rate = expansion_rate - 1;
    columns
        .into_iter()
        .flat_map(|(col, rows)| {
            rows.into_iter().map(move |(row, e_col, e_row)| {
                (col + e_col * expansion_rate, row + e_row * expansion_rate)
            })
        })
        .collect()
}
