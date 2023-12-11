use itertools::Itertools;
use rust_aoc_lib::part2;

#[part2]
pub fn part2(input: &str) -> usize {
    let mut map = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            row.push(char);
        }
        map.push(row);
    }

    let mut where_to_add_rows = Vec::new();

    for (c, line) in map.iter().enumerate() {
        if line.iter().all(|&c| c == '.') {
            where_to_add_rows.push(c);
        }
    }

    let mut where_to_add_cols = Vec::new();

    for i in 0..map[0].len() {
        if map.iter().all(|line| line[i] == '.') {
            where_to_add_cols.push(i);
        }
    }

    let mut galaxy_locations = Vec::new();

    for (y, line) in map.iter().enumerate() {
        for (x, &char) in line.iter().enumerate() {
            if char == '#' {
                galaxy_locations.push((x, y));
            }
        }
    }

    galaxy_locations
        .iter()
        .cartesian_product(galaxy_locations.iter())
        .map(|((x1, y1), (x2, y2))| {
            let x_min = x2.min(x1);
            let x_max = x2.max(x1);

            let y_min = y2.min(y1);
            let y_max = y2.max(y1);

            let mut x_diff = x_max - x_min;
            let mut y_diff = y_max - y_min;

            const EXPANSION: usize = 1000000;

            for row in where_to_add_rows.iter() {
                if row > y_min && row < y_max {
                    y_diff += EXPANSION - 1;
                }
            }

            for col in where_to_add_cols.iter() {
                if col > x_min && col < x_max {
                    x_diff += EXPANSION - 1;
                }
            }

            x_diff + y_diff
        })
        .sum::<usize>()
        / 2
}
