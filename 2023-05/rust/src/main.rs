use std::str::Lines;

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
#[allow(unused_imports)]
use rust_aoc_lib::*;

fn main() {
    let input = include_str!("../input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    // println!("speedy_part_1: {}", speedy_part_1(input));
    println!("speedy_part_2: {}", speedy_part_2(input));

    // assert_eq_same_input!(input, part1, speedy_part_1);
    assert_eq_same_input!(input, part2, speedy_part_2);

    simple_benchmark!(part1, input);
    // simple_benchmark!(speedy_part_1, input);
    // simple_benchmark!(part2, input);
    simple_benchmark!(speedy_part_2, input, 1);
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
        .flat_map(|(start, c)| start..start + c)
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
        .map(|&seed| {
            let mut seed = seed;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    src: usize,
    dst: usize,
    range: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeType {
    Src,
    Dst,
}

impl std::ops::Not for RangeType {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            RangeType::Src => RangeType::Dst,
            RangeType::Dst => RangeType::Src,
        }
    }
}

impl Range {
    /// Notably this is a different order than the text input
    pub fn new(src: usize, dst: usize, range: usize) -> Self {
        Self { src, dst, range }
    }

    pub fn get_next(&self, src: usize) -> usize {
        if (self.src..self.src + self.range).contains(&src) {
            self.dst + src - self.src
        } else {
            src
        }
    }

    pub fn get_range(&self, range_type: RangeType) -> std::ops::Range<usize> {
        match range_type {
            RangeType::Src => self.src..self.src + self.range,
            RangeType::Dst => self.dst..self.dst + self.range,
        }
    }

    pub fn get_max(&self, range_type: RangeType) -> usize {
        match range_type {
            RangeType::Src => self.src + self.range - 1,
            RangeType::Dst => self.dst + self.range - 1,
        }
    }

    pub fn get_min(&self, range_type: RangeType) -> usize {
        match range_type {
            RangeType::Src => self.src,
            RangeType::Dst => self.dst,
        }
    }
}

pub fn combine_maps(a_to_b: &[Range], b_to_c: &[Range]) -> Vec<Range> {
    let mut a_to_c = Vec::new();
    // let mut partial_applications = Vec::new();

    fn check_interactions(
        a_to_c: &mut Vec<Range>,
        to_src_type: RangeType,
        from: &[Range],
        to: &[Range],
    ) {
        for from in from {
            for to in to {
                // if output completely contained in input or vice versa
                if from
                    .get_range(!to_src_type)
                    .contains(&to.get_min(to_src_type))
                    && from
                        .get_range(!to_src_type)
                        .contains(&to.get_max(to_src_type))
                {
                    // This should result in 3 new ranges

                    // 0 .. 5 -> 5 .. 10
                    //           6 .. 9  -> 11 .. 14
                    // Results in
                    // 0 .. 1 -> 5 .. 6
                    // AKA 0 -> 5
                    // and
                    // 1 .. 4 -> 11 .. 14
                    // AKA 1 -> 11, 2 -> 12, 3 -> 13
                    // and
                    // 4 .. 5 -> 9 .. 10
                    // AKA 4 -> 9
                } else if from
                    .get_range(!to_src_type)
                    .contains(&to.get_min(to_src_type))
                {
                    // This should result in 2 new ranges

                    // 0 .. 5 -> 5 .. 10
                    //           7 .. 12  -> 17 .. 22
                    // Results in
                    // 0 .. 2 -> 5 .. 7
                    // AKA 0 -> 5, 1 -> 6
                    // and
                    // 2 .. 5 -> 17 .. 20
                    // AKA 2 -> 17, 3 -> 18, 4 -> 19
                } else if from
                    .get_range(!to_src_type)
                    .contains(&to.get_max(to_src_type))
                {
                    // This should result in 2 new ranges

                    // 0 .. 5 -> 5 .. 10
                    //           3 .. 9  -> 1 .. 7
                    // Results in
                    // 4 .. 5 -> 9 .. 10
                    // AKA 4 -> 9, 5 -> 10
                    // and
                    // 0 .. 4 -> 3 .. 7
                    // AKA 0 -> 3, 1 -> 4, 2 -> 5, 3 -> 6

                    a_to_c.push(Range::new(
                        from.get_max(to_src_type),
                        to.get_min(to_src_type),
                        to.get_min(to_src_type) - from.get_min(to_src_type),
                    ));
                } else {
                    a_to_c.push(*from);
                }
            }
        }
    }

    check_interactions(&mut a_to_c, RangeType::Src, a_to_b, b_to_c);
    // this one might actually need different logic
    // this is due to the case where the 3 .. 6 is partially covered like 4 .. 6
    // so therefore it should only froward 3 .. 4, but current implementation would forward 4 .. 6 differently I think.
    // check_interactions(&mut a_to_c, RangeType::Dst, b_to_c, a_to_b);

    a_to_c
}

#[test]
fn combine_maps_test_no_intersect() {
    let map1 = vec![Range::new(0, 5, 3)]; // 0 -> 5, 1 -> 6, 2 -> 7
    let map2 = vec![Range::new(3, 2, 2)]; // 3 -> 2, 4 -> 3
    let result = combine_maps(&map1, &map2);
    assert_eq!(result, vec![Range::new(0, 5, 3), Range::new(3, 2, 2)]);
}

#[test]
fn combine_maps_test_intersection_contains_no_simple() {
    let map1 = vec![Range::new(0, 5, 3)]; // 0 -> 5, 1 -> 6, 2 -> 7
    let map2 = vec![Range::new(4, 2, 2)]; // 4 -> 2, 5 -> 3
    let result = combine_maps(&map1, &map2);
    // 0 -> 3, 1 -> 6, 2 -> 7, 3 -> 3, 4 -> 2, 5 -> 3
    assert!(!result.contains(map1.first().unwrap()));
    assert!(!result.contains(map2.first().unwrap()));
}

#[test]
fn combine_maps_test_split() {
    let map1 = vec![Range::new(0, 5, 3)]; // 0 -> 5, 1 -> 6, 2 -> 7
    let map2 = vec![Range::new(4, 2, 2)]; // 4 -> 2, 5 -> 3
    let result = combine_maps(&map1, &map2);
    // 0 -> 3, 1 -> 6, 2 -> 7, 3 -> 3, 4 -> 2, 5 -> 3
    assert_eq!(
        result,
        vec![
            Range::new(0, 3, 1),
            Range::new(1, 6, 2),
            Range::new(4, 2, 2)
        ]
    );
}

fn speedy_part_2(input: &str) -> usize {
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
        .flat_map(|&(start, c)| start..start + c)
        .map(|seed| {
            let mut seed = seed;
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

#[test]
fn test_part2() {
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
    assert_eq!(part2(test), 46);
}
