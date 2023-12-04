use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use rust_aoc_lib::*;

fn main() {
    let input = include_str!("../input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("speedy_part_1: {}", speedy_part_1(input));
    // println!("speedy_part_2: {}", speedy_part_2(input));

    assert_eq_same_input!(input, part1, speedy_part_1);
    // assert_eq_same_input!(input, part2, speedy_part_2);

    simple_benchmark!(part1, input, 10000);
    simple_benchmark!(speedy_part_1, input, 10000);
    simple_benchmark!(part2, input, 10000);
    // simple_benchmark!(speedy_part_2, input);
}

fn part1(input: &str) -> usize {
    let mut cards = HashSet::new();
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
                    cards.insert(card);
                });

            ours.split_ascii_whitespace().fold(0, |acc, our| {
                let our = our.parse::<usize>().unwrap();
                if cards.contains(&our) {
                    if acc == 0 {
                        1
                    } else {
                        acc * 2
                    }
                } else {
                    acc
                }
            })
        })
        .sum()
}

fn part2(input: &str) -> usize {
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

            ours.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .filter_map(|x| if cards.contains(&x) { Some(x) } else { None })
                .count()
        })
        .fold(
            (VecDeque::new(), 0),
            |(mut copy_tracker, total): (VecDeque<usize>, usize), x| {
                let copies = copy_tracker.pop_front().unwrap_or(1);

                for i in 0..x {
                    if let Some(val) = copy_tracker.get_mut(i) {
                        *val += copies;
                    } else {
                        copy_tracker.push_back(copies + 1);
                    }
                }

                (copy_tracker, total + copies)
            },
        )
        .1
}

fn speedy_part_1(input: &str) -> usize {
    let mut ans = [0u8; 10];
    let mut nums = [0u8; 25];

    let mut i = 0;
    let mut target: &mut [u8] = &mut ans;
    let mut val = 0;
    let mut total = 0;

    for c in input.as_bytes() {
        match c {
            b'|' => {
                target = &mut nums;
                i = 0;
            }
            b'\n' => {
                target[i] = val;

                let mut points = 0;

                for val in nums.iter().take_while(|val| **val > 0) {
                    if ans.contains(val) {
                        if points == 0 {
                            points = 1;
                        } else {
                            points *= 2;
                        }
                    }
                }

                total += points;

                i = 0;
                val = 0;
                ans = [0u8; 10];
                nums = [0u8; 25];
                target = &mut ans;
            }
            b'0'..=b'9' => {
                val = val * 10 + (c - b'0');
            }
            b' ' => {
                if val > 0 {
                    target[i] = val as u8;
                    i += 1;
                }

                val = 0;
            }
            b':' => {
                val = 0;
            }
            _ => {}
        }
    }

    total
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

#[test]
fn test_speedy_part_1() {
    let test = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    assert_eq!(part1(test), speedy_part_1(test));
}
