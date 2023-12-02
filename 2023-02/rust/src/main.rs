fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));

    let correct_answer = part2(input);

    let now = std::time::Instant::now();
    for _ in 0..100000 {
        assert_eq!(correct_answer, part2(input));
    }
    println!("stefa: {:?}", now.elapsed() / 100000);

    let now = std::time::Instant::now();
    for _ in 0..100000 {
        assert_eq!(correct_answer, part_two(input));
    }
    println!("chase: {:?}", now.elapsed() / 100000);
}

fn is_possible(input: &str) -> (usize, usize, usize, bool) {
    let mut max_seen_red = 1;
    let mut max_seen_green = 1;
    let mut max_seen_blue = 1;

    let possible = input
        .split(", ")
        .map(|item| item.split_once(" ").unwrap())
        .map(|(count, color)| (count.parse::<usize>().unwrap(), color))
        .map(|(count, color)| match color {
            "red" => {
                max_seen_red = max_seen_red.max(count);
                count <= MAX_RED
            }
            "green" => {
                max_seen_green = max_seen_green.max(count);
                count <= MAX_GREEN
            }
            "blue" => {
                max_seen_blue = max_seen_blue.max(count);
                count <= MAX_BLUE
            }
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    (
        max_seen_red,
        max_seen_green,
        max_seen_blue,
        possible.into_iter().all(|x| x),
    )
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
                .map(|input| (is_possible(input)))
                .all(|(_, _, _, possible)| possible)
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
                .map(|input| is_possible(input))
                .fold((1, 1, 1), |acc, (red, green, blue, _)| {
                    (acc.0.max(red), acc.1.max(green), acc.2.max(blue))
                })
        })
        .map(|(max_red, max_green, max_blue)| max_red * max_green * max_blue)
        .sum()
}

fn part_two(puzzle_input: &str) -> usize {
    let x: Vec<_> = puzzle_input
        .lines()
        .map(|line| {
            let (game_id, rules) = line.split_once(':').unwrap();
            let (_, y) = game_id.split_once(' ').unwrap();
            let game_id_num: usize = y.parse().unwrap();
            let rules_list: Vec<_> = rules
                .split(';')
                .map(|round| {
                    round
                        .trim()
                        .split(',')
                        .map(|thing| {
                            let (x, y) = thing.trim().split_once(' ').unwrap();
                            let num: usize = x.parse().unwrap();
                            (num, y)
                        })
                        .collect::<Vec<_>>()
                })
                .collect();

            (game_id_num, rules_list)
        })
        .collect();

    let mut sum = 0;
    for (_, rounds) in &x {
        let mut min_reds = 0;
        let mut min_greens = 0;
        let mut min_blues = 0;

        for round in rounds {
            let mut red_count = 0;
            let mut green_count = 0;
            let mut blue_count = 0;

            for (num, pull_color) in round {
                match *pull_color {
                    "red" => red_count += num,
                    "green" => green_count += num,
                    "blue" => blue_count += num,
                    _ => {
                        panic!("invalid color {}", pull_color);
                    }
                }
            }

            min_reds = min_reds.max(red_count);
            min_greens = min_greens.max(green_count);
            min_blues = min_blues.max(blue_count);
        }

        let power = min_reds * min_greens * min_blues;
        sum += power;
    }

    sum
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
