use rust_aoc_lib::{incr_num, part1, read_number, read_till};

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

#[part1]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|game| game.split_once(": ").unwrap())
        .filter(|(_, second_half)| {
            second_half
                .split("; ")
                .map(|input| {
                    input
                        .split(", ")
                        .map(|item| item.split_once(" ").unwrap())
                        .map(|(count, color)| (count.parse::<usize>().unwrap(), color))
                        .map(|(count, color)| match color {
                            "red" => count <= MAX_RED,
                            "green" => count <= MAX_GREEN,
                            "blue" => count <= MAX_BLUE,
                            _ => unreachable!(),
                        })
                        .all(|x| x)
                })
                .all(|possible| possible)
        })
        .map(|(first_half, _)| {
            first_half
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

// This one is less heavily optimized than part 2, but still faster than part 1 original, by a significant margin.
#[part1]
fn speedy_part_1(input: &str) -> usize {
    let mut success: usize = 0;

    let mut iter = input.as_bytes().iter();

    'outer: while let Some(_) = iter.next() {
        let mut val = 0;

        read_till!(iter, b' ');

        let game_num = read_number!(iter);

        while let Some(&c) = iter.next() {
            match c {
                b'0'..=b'9' => {
                    incr_num!(val, c);
                }
                b'r' => {
                    if val > MAX_RED {
                        break;
                    }

                    val = 0;
                }
                b'g' => {
                    if val > MAX_GREEN {
                        break;
                    }

                    val = 0;
                }
                b'b' => {
                    if val > MAX_BLUE {
                        break;
                    }

                    val = 0;
                }
                b'\n' => {
                    success += game_num;
                    continue 'outer;
                }
                _ => {}
            }
        }

        if iter.next().is_none() {
            success += game_num;
        }

        while !matches!(iter.next(), Some(b'\n') | None) {}
    }

    success
}
