use std::str::Lines;

use rust_aoc_lib::part1;

pub fn parse_line(line: &str) -> (usize, usize, usize) {
    let mut parts = line.split_whitespace();
    let a = parts.next().unwrap().parse::<usize>().unwrap();
    let b = parts.next().unwrap().parse::<usize>().unwrap();
    let c = parts.next().unwrap().parse::<usize>().unwrap();
    (a, b, c)
}

pub fn read_till_empty_line(lines: &mut Lines) -> Vec<(usize, usize, usize)> {
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

#[part1]
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
