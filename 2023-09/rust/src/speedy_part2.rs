use itertools::Itertools;
use rust_aoc_lib::part2;

#[part2]
pub fn speedy_part2(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            std::iter::successors(
                Some(
                    line.split_ascii_whitespace()
                        .map(|x| x.parse::<isize>().unwrap())
                        .collect_vec(),
                ),
                |prev| {
                    (!prev.iter().all(|&x| x == 0))
                        .then(|| prev.windows(2).map(|arr| arr[1] - arr[0]).collect_vec())
                },
            )
            .collect_vec()
            .iter()
            .rev()
            .skip(1)
            .map(|x| x.first().unwrap())
            .fold(0, |acc, diff| diff - acc)
        })
        .sum()
}
