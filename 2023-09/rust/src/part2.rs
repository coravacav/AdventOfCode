use itertools::Itertools;
use rust_aoc_lib::part2;

#[part2]
pub fn part2(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let history = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect_vec();

            let mut differences: Vec<Vec<isize>> = vec![];
            differences.push(history);

            while !differences.last().unwrap().iter().all(|x| *x == 0) {
                let new_differences = differences
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|arr| (arr[1] - arr[0]))
                    .collect_vec();

                differences.push(new_differences);
            }

            let mut start = 0;

            for diff in differences.iter().rev().skip(1) {
                start = diff.first().unwrap() - start;
            }

            start
        })
        .sum()
}
