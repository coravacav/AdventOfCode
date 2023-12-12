use bit_vec::BitVec;
use itertools::Itertools;
use rust_aoc_lib::part2;

fn parse(row: &str) -> (Vec<char>, Vec<u8>) {
    let (cell_str, num_str) = row.split_once(' ').unwrap();

    let numbers = num_str
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect_vec();

    (cell_str.chars().collect_vec(), numbers)
}

fn generate_all_possibilities(cells: &[char], mut numbers: &[u8], number_of_damaged: u8) -> usize {
    let Some(cell) = cells.first() else {
        return 0;
    };

    if numbers.is_empty() {
        return 0;
    }

    if Some(&number_of_damaged) == numbers.first() {
        numbers = &numbers[1..];

        if numbers.is_empty() {
            return if cells.contains(&'#') { 0 } else { 1 };
        }
    }

    match cell {
        '#' => {
            if number_of_damaged == u8::MAX {
                return 0;
            }

            generate_all_possibilities(&cells[1..], &numbers[1..], number_of_damaged + 1)
        }
        '?' => {
            generate_all_possibilities(&cells[1..], &numbers[1..], number_of_damaged + 1)
                + generate_all_possibilities(&cells[1..], &numbers[1..], 0)
        }
        '.' => {
            if number_of_damaged > 0 {
                return 0;
            }

            generate_all_possibilities(&cells[1..], &numbers[1..], 0)
        }
        _ => unreachable!(),
    }
}

#[part2]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(parse)
        .map(|(cells, numbers)| {
            let cells_len = cells.len() * 5;
            let numbers_len = numbers.len() * 5;

            (
                cells.into_iter().cycle().take(cells_len).collect_vec(),
                numbers.into_iter().cycle().take(numbers_len).collect_vec(),
            )
        })
        .inspect(|(cells, numbers)| {
            for cell in cells.iter() {
                print!("{} ", cell);
            }
            for (i, num) in numbers.iter().enumerate() {
                print!("{}", num);
                if i != numbers.len() - 1 {
                    print!(",");
                }
            }
            println!();
        })
        .map(|(cells, numbers)| generate_all_possibilities(&cells, &numbers, 0))
        // .inspect(|n| println!("{}", n))
        .sum()
}

fn pretty_print_bitvec(a: &BitVec) -> String {
    a.iter().map(|b| if b { '#' } else { '.' }).collect()
}
