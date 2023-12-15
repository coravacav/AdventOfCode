use rust_aoc_lib::part1;

#[part1]
pub fn part1(input: &str) -> usize {
    input
        .as_bytes()
        .split(|c| *c == b',')
        .map(|c| {
            let mut value = 0;
            for &c in c {
                if c == b'\n' {
                    continue;
                }

                value += c as usize;
                value *= 17;
                value %= 256;
            }

            value
        })
        .sum()
}
