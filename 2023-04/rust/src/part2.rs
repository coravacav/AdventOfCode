use std::collections::VecDeque;

use rust_aoc_lib::part2;

#[part2]
fn part2(input: &str) -> usize {
    let mut cards = Vec::new();
    input
        .lines()
        .map(|line| line.split_once(":").unwrap().1)
        .map(|line| line.split_once(" | ").unwrap())
        .map(|(winners, ours)| {
            cards.clear();
            winners
                .split_ascii_whitespace()
                .map(|win| win.parse::<usize>().unwrap())
                .for_each(|card| {
                    cards.push(card);
                });

            ours.split_ascii_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .filter_map(|x| if cards.contains(&x) { Some(x) } else { None })
                .count()
        })
        .fold(
            (VecDeque::new(), 0),
            |(mut copy_tracker, total): (VecDeque<usize>, usize), x| {
                let copies = copy_tracker.pop_front().unwrap_or(1);

                for i in 0..x {
                    match copy_tracker.get_mut(i) {
                        Some(val) => *val += copies,
                        None => copy_tracker.push_back(copies),
                    }
                }

                (copy_tracker, total + copies)
            },
        )
        .1
}
