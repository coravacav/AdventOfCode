use crate::part1::read_till_empty_line;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rust_aoc_lib::part2;

#[part2]
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
