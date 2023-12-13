use itertools::Itertools;
use rust_aoc_lib::part2;

fn compare_lines((line1, line2): (&[u8], &[u8])) -> bool {
    line1
        .iter()
        .rev()
        .take(line2.len())
        .zip(line2.iter().take(line1.len()))
        .all(|(a, b)| a == b)
}

fn check_symmetry(lines: &[u8], l: usize) -> Option<usize> {
    (1..l)
        .filter(|&i| compare_lines(lines[0..l].split_at(i)))
        .chain((1..l).filter(|&i| compare_lines(lines[l..2 * l].split_at(i))))
        .find(|&i| {
            lines
                .chunks(l)
                .filter(|line| !compare_lines(line.split_at(i)))
                .count()
                == 1
        })
}

#[part2]
pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| group.as_bytes())
        .map(|lines| {
            let starting_len = lines.len();
            let lines = lines.iter().filter(|&&b| b != b'\n').copied().collect_vec();
            let ending_len = lines.len();

            let m = starting_len - ending_len + 1;
            let n = lines.len() / m;

            if let Some(i) = check_symmetry(&lines, n) {
                return i;
            }

            // transpose lines
            let lines = (0..n)
                .flat_map(|i| lines.chunks(n).map(|line| line[i]).collect::<Vec<_>>())
                .collect::<Vec<_>>();

            if let Some(i) = check_symmetry(&lines, m) {
                return i * 100;
            }

            panic!("No symmetry found")
        })
        .sum()
}
