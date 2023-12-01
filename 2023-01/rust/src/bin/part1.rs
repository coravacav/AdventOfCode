use tap::Pipe;

fn main() {
    let input = include_str!("../../input.txt");
    println!("part1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.chars().filter(|&c| c.is_digit(10)).pipe_ref_mut(|s| {
                format!("{}{}", s.nth(0).unwrap_or('0'), s.last().unwrap_or('0'))
                    .parse::<usize>()
                    .unwrap_or(0)
            })
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(
            r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#
        ),
        142
    );
}
