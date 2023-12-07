use itertools::Itertools;
use rust_aoc_lib::part1;

#[part1]
fn part1(input: &str) -> usize {
    let both = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .filter_map(Result::ok)
        .collect_vec();

    let (time, distance) = both.split_at(both.len() / 2);

    fn passes_distance(pressed_time: usize, total_time: usize, distance: usize) -> usize {
        let speed = pressed_time;
        let remaining_time = total_time - pressed_time;
        let covered_distance = speed * remaining_time;
        if covered_distance > distance {
            1
        } else {
            0
        }
    }

    time.iter()
        .zip(distance.iter())
        .map(|(&time, &distance)| {
            (0..time)
                .map(|pressed_time| passes_distance(pressed_time, time, distance))
                .sum::<usize>()
        })
        .product::<usize>()
}
