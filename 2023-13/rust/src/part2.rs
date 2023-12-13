use itertools::Itertools;
use rust_aoc_lib::part2;

#[part2]
pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        // .inspect(|group| println!("{}!", group))
        .map(|group| {
            let lines = group.lines().collect::<Vec<_>>();

            // Check for vertical symmetry

            fn compare_lines((line1, line2): (&str, &str)) -> bool {
                let line2_reversed = line2.chars().rev().collect::<String>();
                if line1.len() > line2.len() {
                    line1.ends_with(&line2_reversed)
                } else {
                    line2_reversed.ends_with(line1)
                }
            }

            let line_zero_possibilities = (1..lines[0].len())
                .filter(|&i| compare_lines(lines[0].split_at(i)))
                .collect::<Vec<_>>();

            let line_one_possibilities = (1..lines[1].len())
                .filter(|&i| compare_lines(lines[1].split_at(i)))
                .collect::<Vec<_>>();

            let possible_vertical_symmetry = line_zero_possibilities
                .iter()
                .chain(line_one_possibilities.iter())
                .unique()
                .collect::<Vec<_>>();

            if let Some(&i) = possible_vertical_symmetry.iter().find(|&&i| {
                lines
                    .iter()
                    .filter(|line| !compare_lines(line.split_at(*i)))
                    .count()
                    == 1
            }) {
                return *i;
            }

            // transpose lines
            let lines = (0..lines[0].len())
                .map(|i| {
                    lines
                        .iter()
                        .map(|line| line.chars().nth(i).unwrap())
                        .collect::<String>()
                })
                .collect::<Vec<_>>();

            let line_zero_possibilities = (1..lines[0].len())
                .filter(|&i| compare_lines(lines[0].split_at(i)))
                .collect::<Vec<_>>();

            let line_one_possibilities = (1..lines[1].len())
                .filter(|&i| compare_lines(lines[1].split_at(i)))
                .collect::<Vec<_>>();

            let possible_horizontal_symmetry = line_zero_possibilities
                .iter()
                .chain(line_one_possibilities.iter())
                .unique()
                .collect::<Vec<_>>();

            if let Some(&i) = possible_horizontal_symmetry.iter().find(|&&i| {
                lines
                    .iter()
                    .filter(|line| !compare_lines(line.split_at(*i)))
                    .count()
                    == 1
            }) {
                return i * 100;
            }

            println!("{}", group);

            panic!("No symmetry found")
        })
        .sum()
}
