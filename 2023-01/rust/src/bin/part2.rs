#![allow(dead_code)]

fn main() {
    let input = include_str!("../../input.txt");
    let answer = part_two(input);

    let now = std::time::Instant::now();
    for _ in 0..1000 {
        assert_eq!(part_two(input), answer);
    }
    println!("time: {:?}", now.elapsed() / 1000);
    let now = std::time::Instant::now();
    for _ in 0..1000 {
        assert_eq!(part2(input), answer);
    }
    println!("time: {:?}", now.elapsed() / 1000);
}

macro_rules! check_and_break_start {
    ($input:ident, $str:literal, $num:literal) => {
        if $input.starts_with($str) {
            $input = &$input[$str.len()..];
            break Some($num);
        }
    };
}

macro_rules! check_and_break_end {
    ($input:ident, $str:literal, $num:literal) => {
        if $input.ends_with($str) {
            break Some($num);
        }
    };
}

fn loop_over_and_get_ints(mut input: &str) -> (Option<usize>, Option<usize>) {
    (
        loop {
            if input.is_empty() {
                return (None, None);
            }

            check_and_break_start!(input, "one", 1);
            check_and_break_start!(input, "two", 2);
            check_and_break_start!(input, "three", 3);
            check_and_break_start!(input, "four", 4);
            check_and_break_start!(input, "five", 5);
            check_and_break_start!(input, "six", 6);
            check_and_break_start!(input, "seven", 7);
            check_and_break_start!(input, "eight", 8);
            check_and_break_start!(input, "nine", 9);

            if let Ok(u) = input[0..1].parse::<usize>() {
                break Some(u);
            }

            input = &input[1..];
        },
        loop {
            if input.is_empty() {
                return (None, None);
            }

            check_and_break_end!(input, "one", 1);
            check_and_break_end!(input, "two", 2);
            check_and_break_end!(input, "three", 3);
            check_and_break_end!(input, "four", 4);
            check_and_break_end!(input, "five", 5);
            check_and_break_end!(input, "six", 6);
            check_and_break_end!(input, "seven", 7);
            check_and_break_end!(input, "eight", 8);
            check_and_break_end!(input, "nine", 9);

            if let Ok(u) = input[input.len() - 1..input.len()].parse::<usize>() {
                break Some(u);
            }

            input = &input[..input.len() - 1];
        },
    )
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| match loop_over_and_get_ints(line) {
            (Some(first), Some(last)) => first * 10 + last,
            _ => 0,
        })
        .sum()
}

fn part_two(puzzle_input: &str) -> usize {
    let nums = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    puzzle_input
        .lines()
        .map(|line| {
            let line_chars: Vec<char> = line.chars().collect();

            let word_stuff_left = nums
                .iter()
                .map(|(word, value)| (word, value, line.find(word)))
                .min_by_key(|(_, _, idx)| idx.unwrap_or(usize::MAX))
                .unwrap();
            let word_stuff_right = nums
                .iter()
                .map(|(word, value)| (word, value, line.rfind(word)))
                .max_by_key(|(_, _, idx)| idx.unwrap_or(usize::MIN))
                .unwrap();

            let left_most_digit = line.find(|c: char| c.is_ascii_digit());
            let right_most_digit = line.rfind(|c: char| c.is_ascii_digit());

            let first_digit = if word_stuff_left.2.unwrap_or(usize::MAX)
                < left_most_digit.unwrap_or(usize::MAX)
            {
                *word_stuff_left.1
            } else {
                line_chars
                    .get(left_most_digit.unwrap())
                    .unwrap()
                    .to_string()
                    .parse()
                    .unwrap()
            };

            let second_digit = if word_stuff_right.2.unwrap_or(usize::MIN)
                > right_most_digit.unwrap_or(usize::MIN)
            {
                *word_stuff_right.1
            } else {
                line_chars
                    .get(right_most_digit.unwrap())
                    .unwrap()
                    .to_string()
                    .parse()
                    .unwrap()
            };

            (first_digit * 10) + second_digit
        })
        .sum()
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(
            r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#,
        ),
        281
    );
}
