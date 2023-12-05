use std::{collections::HashMap, str::Lines};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
#[allow(unused_imports)]
use rust_aoc_lib::*;

fn main() {
    let input = include_str!("../input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    // println!("speedy_part_1: {}", speedy_part_1(input));
    // println!("speedy_part_2: {}", speedy_part_2(input));

    // assert_eq_same_input!(input, part1, speedy_part_1);
    // assert_eq_same_input!(input, part2, speedy_part_2);

    simple_benchmark!(part1, input);
    // simple_benchmark!(speedy_part_1, input);
    // simple_benchmark!(part2, input);
    // simple_benchmark!(speedy_part_2, input);
}

fn parse_line(line: &str) -> (usize, usize, usize) {
    let mut parts = line.split_whitespace();
    let a = parts.next().unwrap().parse::<usize>().unwrap();
    let b = parts.next().unwrap().parse::<usize>().unwrap();
    let c = parts.next().unwrap().parse::<usize>().unwrap();
    (a, b, c)
}

fn read_till_empty_line(lines: &mut Lines) -> Vec<(usize, usize, usize)> {
    let mut result = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            lines.next(); // We know that the next one must be a label, we don't actually need it
            break;
        }
        result.push(parse_line(line));
    }
    result
}

// Any source numbers that aren't mapped correspond to the same destination number. So, seed number 10 corresponds to soil number 10.

fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    // After the initial list, we have a blank line and a label, we don't need either
    lines.next();
    lines.next();

    let seed_to_soil = read_till_empty_line(&mut lines);
    let soil_to_fertilizer = read_till_empty_line(&mut lines);
    let fertilizer_to_water = read_till_empty_line(&mut lines);
    let water_to_light = read_till_empty_line(&mut lines);
    let light_to_temperature = read_till_empty_line(&mut lines);
    let temperature_to_humidity = read_till_empty_line(&mut lines);
    let humidity_to_location = read_till_empty_line(&mut lines);

    // Follow all vec values through the maps and see where they end up
    // If result / range not in vec, then it's the same as the input
    let mut result = Vec::new();
    for mut seed in seeds {
        for mapping in [
            &seed_to_soil,
            &soil_to_fertilizer,
            &fertilizer_to_water,
            &water_to_light,
            &light_to_temperature,
            &temperature_to_humidity,
            &humidity_to_location,
        ] {
            let mut location = None;
            for (dst, src, range) in mapping.iter() {
                if (*src..src + range).contains(&seed) {
                    location = Some(dst + seed - src);
                    break;
                }
            }
            if let Some(location) = location {
                seed = location;
            }
        }
        result.push(seed);
    }

    result.into_iter().min().unwrap()
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    // seeds now map to ranges

    let seeds = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|mut c| (c.next().unwrap(), c.next().unwrap()))
        .flat_map(|(a, b)| a..a + b)
        .collect_vec();

    // After the initial list, we have a blank line and a label, we don't need either
    lines.next();
    lines.next();

    let seed_to_soil = read_till_empty_line(&mut lines);
    let soil_to_fertilizer = read_till_empty_line(&mut lines);
    let fertilizer_to_water = read_till_empty_line(&mut lines);
    let water_to_light = read_till_empty_line(&mut lines);
    let light_to_temperature = read_till_empty_line(&mut lines);
    let temperature_to_humidity = read_till_empty_line(&mut lines);
    let humidity_to_location = read_till_empty_line(&mut lines);

    // Follow all vec values through the maps and see where they end up
    // If result / range not in vec, then it's the same as the input

    seeds
        .par_iter()
        .map(|seed| {
            let mut seed = *seed;
            for mapping in [
                &seed_to_soil,
                &soil_to_fertilizer,
                &fertilizer_to_water,
                &water_to_light,
                &light_to_temperature,
                &temperature_to_humidity,
                &humidity_to_location,
            ] {
                let mut location = None;
                for (dst, src, range) in mapping.iter() {
                    if (*src..src + range).contains(&seed) {
                        location = Some(dst + seed - src);
                        break;
                    }
                }
                if let Some(location) = location {
                    seed = location;
                }
            }
            seed
        })
        .min()
        .unwrap()
}

#[test]
fn test_part1() {
    let test = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
    assert_eq!(part1(test), 35);
}

// #[test]
// fn test_part2() {
//     let test = r#""#;
//     assert_eq!(part2(test), ___);
// }
