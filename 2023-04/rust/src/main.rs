use std::collections::{vec_deque, HashSet, VecDeque};

use itertools::Itertools;
#[allow(unused_imports)]
use rust_aoc_lib::*;

fn main() {
    let input = include_str!("../input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    // println!("speedy_part_1: {}", speedy_part_1(input));
    // println!("speedy_part_2: {}", speedy_part_2(input));

    // assert_eq_same_input!(input, part1, speedy_part_1);
    // assert_eq_same_input!(input, part2, speedy_part_2);

    // simple_benchmark!(part1, input);
    // simple_benchmark!(speedy_part_1, input);
    // simple_benchmark!(part2, input);
    // simple_benchmark!(speedy_part_2, input);
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(":").unwrap();
            let mut cards = HashSet::new();
            let (winners, ours) = line.split_once(" | ").unwrap();
            let winners = winners.trim();
            let ours = ours.trim();

            for win in winners.split_whitespace() {
                let win = win.parse::<usize>().unwrap();
                cards.insert(win);
            }

            let mut points = 0;

            for our in ours.split_whitespace() {
                let our = our.parse::<usize>().unwrap();
                if cards.contains(&our) {
                    if points == 0 {
                        points = 1;
                    } else {
                        points *= 2;
                    }
                }
            }

            points
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let res = input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(":").unwrap();
            let mut cards = HashSet::new();
            let (winners, ours) = line.split_once(" | ").unwrap();
            let winners = winners.trim();
            let ours = ours.trim();

            for win in winners.split_whitespace() {
                let win = win.parse::<usize>().unwrap();
                cards.insert(win);
            }

            ours.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .filter_map(|x| if cards.contains(&x) { Some(x) } else { None })
                .count()
        })
        .collect_vec();

    let mut c = VecDeque::with_capacity(res.len());
    for _ in 0..res.len() {
        c.push_back(1);
    }

    res.iter()
        .fold(
            (c, 0),
            |(mut copy_tracker, mut total): (VecDeque<usize>, usize), &x| {
                let copies = copy_tracker.pop_front().unwrap_or(1);

                total += copies;

                if x == 0 {
                    (copy_tracker, total)
                } else {
                    for i in 0..x {
                        copy_tracker[i] += copies;
                    }

                    (copy_tracker, total)
                }
            },
        )
        .1
}

#[test]
fn test_part1() {
    let test = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    assert_eq!(part1(test), 13);
}

#[test]
fn test_part2() {
    let test = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    assert_eq!(part2(test), 30);
}
