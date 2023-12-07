use rust_aoc_lib::{incr_num, part2};

#[part2]
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

// Thanks to members of ThePrimeagens's discord for giving ideas.
// Still optimized plenty on my own, but nonetheless, thanks.
#[part2]
fn speedy_part_2(input: &str) -> usize {
    let mut success: usize = 0;
    let (mut val, mut r, mut g, mut b) = (0usize, 0usize, 0usize, 0usize);

    for c in input.as_bytes() {
        match c {
            b'0'..=b'9' => {
                incr_num!(val, c);
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
            b'\n' => {
                success += r * g * b;
                (val, r, g, b) = (0, 0, 0, 0);
            }
            b':' => {
                val = 0;
            }
            _ => {}
        }
    }

    success + r * g * b
}
