use rust_aoc_lib::part2;
use std::hint::unreachable_unchecked;

#[part2]
pub fn part2(input: &str) -> usize {
    input
        .as_bytes()
        .split(|c| *c == b',')
        .map(|c| {
            let mut value = 0;
            let mut hash = 0;
            let mut iter = c.iter().filter(|&&c| c != b'\n').peekable();

            while let Some(c) = iter.next() {
                match c {
                    b'-' => return (*c, value, hash, 0),
                    b'=' => return (*c, value, hash, iter.next().map(|c| *c - b'0').unwrap()),
                    c => {
                        hash = hash << 8 | *c as usize;
                        value = ((value + *c as usize) * 17) % 256;
                    }
                }
            }

            unsafe { unreachable_unchecked() }
        })
        .fold(
            vec![vec![]; 256],
            |mut acc: Vec<Vec<usize>>, (c, value, hash, key)| {
                match c {
                    b'-' => acc[value].retain(|c| c >> 8 != hash),
                    b'=' => {
                        if let Some(v) = acc[value].iter_mut().find(|c| **c >> 8 == hash) {
                            *v = *v >> 8 << 8 | key as usize;
                        } else {
                            acc[value].push(hash << 8 | key as usize);
                        }
                    }

                    _ => unsafe { unreachable_unchecked() },
                }

                acc
            },
        )
        .iter()
        .enumerate()
        .filter(|(_, v)| !v.is_empty())
        .map(|(i, v)| {
            v.iter()
                .enumerate()
                .map(|(x, c)| (i + 1) * (c & 0b_1111_1111) * (x + 1))
                .sum::<usize>()
        })
        .sum()
}
