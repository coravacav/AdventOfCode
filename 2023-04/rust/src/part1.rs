use rust_aoc_lib::part1;

#[part1]
fn part1(input: &str) -> usize {
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
                .map(|our| our.parse::<usize>().unwrap())
                .fold(0, |acc, our| match (cards.contains(&our), acc) {
                    (false, _) => acc,
                    (_, 0) => 1,
                    _ => acc * 2,
                })
        })
        .sum()
}

#[part1]
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
