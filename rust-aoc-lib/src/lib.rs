pub mod macros;
pub use macros::*;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PartNum {
    Part1,
    Part2,
}

pub struct IterationStats {
    pub iterations: u128,
    pub result: usize,
}

pub struct Stats<'a> {
    pub name: &'a str,
    pub part_num: PartNum,
    pub iteration_stats: IterationStats,
    pub implementation: &'a PartImplementation,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct PartImplementation {
    pub part_num: PartNum,
    pub name: &'static str,
    pub fn_ptr: fn(&str) -> usize,
}

impl PartImplementation {
    pub const fn new(part_num: PartNum, name: &'static str, fn_ptr: fn(&str) -> usize) -> Self {
        Self {
            part_num,
            name,
            fn_ptr,
        }
    }

    fn run(&self, input: &str) -> usize {
        (self.fn_ptr)(input)
    }

    fn get_iterations(&self, input: &str) -> IterationStats {
        let start = std::time::Instant::now();
        let result = self.run(std::hint::black_box(input));

        let iterations = Duration::from_secs(2).as_nanos() / start.elapsed().as_nanos();

        let multiple_of_ten_below = 10u128.pow((iterations as f64).log10().floor() as u32 - 1);

        // round iterations to the nearest multiple of 10 below
        let iterations = if iterations > 1000 {
            iterations - iterations % multiple_of_ten_below + multiple_of_ten_below
        } else {
            multiple_of_ten_below
        };

        IterationStats { iterations, result }
    }

    pub fn get_stats(&self, input: &str) -> Stats {
        Stats {
            name: self.name,
            part_num: self.part_num,
            iteration_stats: self.get_iterations(input),
            implementation: self,
        }
    }

    pub fn benchmark(&self, input: &str, iterations: u128) -> Duration {
        let now = std::time::Instant::now();

        for _ in 0..iterations {
            self.run(std::hint::black_box(input));
        }

        let now = now.elapsed();

        Duration::from_nanos((now.as_nanos() / iterations).try_into().unwrap())
    }
}

#[macro_export]
macro_rules! setup_distributed {
    () => {
        #[linkme::distributed_slice]
        pub static ALL_IMPLEMENTATIONS: [rust_aoc_lib::PartImplementation];

        pub fn do_all(input: &str) {
            use itertools::Itertools;
            use rust_aoc_lib::{yansi, PartNum};

            println!("{}", yansi::Paint::new("computing print stats...").dimmed());

            let stats = ALL_IMPLEMENTATIONS
                .iter()
                .map(|p| p.get_stats(input))
                .collect_vec();

            let longest_name = stats.iter().map(|stats| stats.name.len()).max().unwrap();

            let part1 = stats
                .iter()
                .filter(|p| p.part_num == PartNum::Part1)
                .sorted_by_key(|p| p.name)
                .collect_vec();

            let part2 = stats
                .iter()
                .filter(|p| p.part_num == PartNum::Part2)
                .sorted_by_key(|p| p.name)
                .collect_vec();

            println!("{}", yansi::Paint::new("Answers: ").dimmed());

            let parts = [part1, part2];

            for part in &parts {
                for part in part {
                    println!(
                        "{:>width$}: {}",
                        yansi::Paint::new(part.name).bold(),
                        (part.iteration_stats).result,
                        width = longest_name,
                    );
                }
            }

            // Assert all answers are the same
            parts.iter().for_each(|part| {
                part.iter()
                    .map(|p| p.iteration_stats.result)
                    .reduce(|a, b| {
                        assert_eq!(a, b);
                        a
                    });
            });

            println!("{}", yansi::Paint::new("Benchmarks: ").dimmed());

            for part in &parts {
                part.iter()
                    .map(|stats| {
                        (
                            stats,
                            stats
                                .implementation
                                .benchmark(input, stats.iteration_stats.iterations),
                        )
                    })
                    .for_each(|(stats, run_time)| {
                        println!(
                            "{:>width$}: {:time$} {}",
                            yansi::Paint::new(stats.name).bold(),
                            yansi::Color::Cyan.paint(format!("{:?}", run_time)).bold(),
                            yansi::Paint::new(format!(
                                "{:iterations$} iterations",
                                stats.iteration_stats.iterations,
                                iterations = 10
                            ))
                            .dimmed(),
                            width = longest_name,
                            time = 10, // Dunno if I wanna bother actually calculating this
                        );
                    });
            }
        }
    };
}

pub use linkme;
pub use proc_macros::*;
pub use yansi;
