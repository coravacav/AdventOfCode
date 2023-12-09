use solution_2023_09::{do_all, part2::part2};

fn main() {
    let input = include_str!("../input.txt");

    println!("Part 2: {}", part2(input));

    do_all(input);
}
