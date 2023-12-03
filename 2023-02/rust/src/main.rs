fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("speedy_part_1: {}", speedy_part_1(input));
    println!("part2: {}", part2(input));
    println!("speedy_part_2: {}", speedy_part_2(input));

    assert_eq!(part1(input), speedy_part_1(input));
    assert_eq!(part2(input), speedy_part_2(input));

    let now = std::time::Instant::now();
    for _ in 0..100000 {
        part1(input);
    }
    println!("part1: {:?}", now.elapsed() / 100000);

    let now = std::time::Instant::now();
    for _ in 0..100000 {
        speedy_part_1(input);
    }
    println!("speedy_part_1: {:?}", now.elapsed() / 100000);

    let now = std::time::Instant::now();
    for _ in 0..100000 {
        part2(input);
    }
    println!("part2: {:?}", now.elapsed() / 100000);

    let now = std::time::Instant::now();
    for _ in 0..100000 {
        speedy_part_2(input);
    }
    println!("speedy_part_2: {:?}", now.elapsed() / 100000);
}

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

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

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|game| game.split_once(": ").unwrap())
        .map(|(_, second_half)| {
            second_half
                .split("; ")
                .map(|input| {
                    input
                        .split(", ")
                        .map(|item| item.split_once(" ").unwrap())
                        .map(|(count, color)| (count.parse::<usize>().unwrap(), color))
                        .map(|(count, color)| match color {
                            "red" => (count, 1, 1),
                            "green" => (1, count, 1),
                            "blue" => (1, 1, count),
                            _ => unreachable!(),
                        })
                        .fold((1, 1, 1), |a, b| (a.0.max(b.0), a.1.max(b.1), a.2.max(b.2)))
                })
                .fold((1, 1, 1), |a, b| (a.0.max(b.0), a.1.max(b.1), a.2.max(b.2)))
        })
        .map(|(max_red, max_green, max_blue)| max_red * max_green * max_blue)
        .sum()
}

// This one is less heavily optimized than part 2, but still faster than part 1 original, by a significant margin.
fn speedy_part_1(input: &str) -> usize {
    let mut success: usize = 0;

    let mut iter = input.as_bytes().iter();

    'outer: while let Some(_) = iter.next() {
        let mut game_num = 0;
        let mut val = 0;

        // read till space
        while !matches!(iter.next(), Some(b' ')) {}

        while let Some(&c @ b'0'..=b'9') = iter.next() {
            game_num = game_num * 10 + (c - b'0') as usize;
        }

        while let Some(&c) = iter.next() {
            match c {
                b'0'..=b'9' => {
                    val = val * 10 + (c - b'0') as usize;
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

// Thanks to members of ThePrimeagens's discord for giving ideas.
// Still largely my own, but nonetheless, thanks.
fn speedy_part_2(input: &str) -> usize {
    let mut success: usize = 0;
    let (mut val, mut r, mut g, mut b) = (0usize, 0usize, 0usize, 0usize);

    for c in input.as_bytes() {
        match c {
            b'0'..=b'9' => {
                val = val * 10 + (c - b'0') as usize;
            }
            b'r' => {
                r = r.max(val);
                val = 0;
            }
            b'g' => {
                g = g.max(val);
                val = 0;
            }
            b'b' => {
                b = b.max(val);
                val = 0;
            }
            b'\n' | b':' => {
                success += r * g * b;
                (val, r, g, b) = (0, 0, 0, 0);
            }
            _ => {}
        }
    }

    success + r * g * b
}

#[test]
fn test_part2() {
    let test = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    assert_eq!(part2(test), 2286);
}
