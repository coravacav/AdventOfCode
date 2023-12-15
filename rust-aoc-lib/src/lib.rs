pub mod macros;
pub use macros::*;
use std::time::Duration;

pub struct IterationStats {
    pub iterations: u128,
    pub result: RetType,
}

pub struct Stats<'a> {
    pub name: &'a str,
    pub part_num: usize,
    pub iteration_stats: IterationStats,
    pub implementation: &'a PartImplementation,
}

pub struct InitImplementation {
    pub fn_ptr: fn(),
}

impl InitImplementation {
    pub const fn new(fn_ptr: fn()) -> Self {
        Self { fn_ptr }
    }
}

#[allow(non_camel_case_types)]
#[derive(Ord, Debug, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum RetType {
    usize(usize),
    isize(isize),
}

impl std::fmt::Display for RetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RetType::usize(x) => write!(f, "{}", x),
            RetType::isize(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct PartImplementation {
    pub part_num: usize,
    pub name: &'static str,
    pub fn_ptr: fn(&str) -> RetType,
}

impl PartImplementation {
    pub const fn new(part_num: usize, name: &'static str, fn_ptr: fn(&str) -> RetType) -> Self {
        Self {
            part_num,
            name,
            fn_ptr,
        }
    }

    fn run(&self, input: &str) -> RetType {
        (self.fn_ptr)(input)
    }

    fn get_iterations(&self, input: &str) -> IterationStats {
        let start = std::time::Instant::now();
        let result = self.run(std::hint::black_box(input));
        let time = start.elapsed();

        if time > Duration::from_secs(5) {
            return IterationStats {
                iterations: 1,
                result,
            };
        }

        let iterations = Duration::from_secs(5).as_nanos() / time.as_nanos();

        let multiple_of_ten_below = 10u128.pow((iterations as f64).log10().floor() as u32 - 1);

        // round iterations to the nearest multiple of 10 below
        let iterations = iterations - iterations % multiple_of_ten_below + multiple_of_ten_below;

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
    // Want to be able to define multiple "all implementations"
    () => {
        #[linkme::distributed_slice]
        pub static ALL_IMPLEMENTATIONS: [rust_aoc_lib::PartImplementation];

        #[linkme::distributed_slice]
        pub static ALL_INITS: [rust_aoc_lib::InitImplementation];

        pub fn do_init() {
            ALL_INITS.iter().for_each(|init| (init.fn_ptr)());
        }

        pub fn do_all(input: &str) {
            use itertools::Itertools;
            use rust_aoc_lib::yansi;

            println!("{}", yansi::Paint::new("computing print stats...").dimmed());

            let stats = ALL_IMPLEMENTATIONS
                .iter()
                .map(|p| p.get_stats(input))
                .collect_vec();

            let longest_name = stats.iter().map(|stats| stats.name.len()).max().unwrap();

            let part1 = stats
                .iter()
                .filter(|p| p.part_num == 1)
                .sorted_by_key(|p| p.name)
                .collect_vec();

            let part2 = stats
                .iter()
                .filter(|p| p.part_num == 2)
                .sorted_by_key(|p| p.name)
                .collect_vec();

            println!("{}", yansi::Paint::new("Answers: ").dimmed());

            let parts = [part1, part2];

            parts.iter().for_each(|part| {
                part.iter()
                    .map(|part| {
                        println!(
                            "{:>width$}: {}",
                            yansi::Paint::new(part.name).bold(),
                            (part.iteration_stats).result,
                            width = longest_name,
                        );

                        part.iteration_stats.result
                    })
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
