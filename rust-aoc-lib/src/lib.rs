pub mod macros;
pub use macros::*;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PartNum {
    Part1,
    Part2,
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

    pub fn run(&self, input: &str) -> usize {
        (self.fn_ptr)(input)
    }

    pub fn benchmark(&self, input: &str) {
        let mut iterations = 1;

        let start = std::time::Instant::now();

        for _ in 0..iterations {
            (self.fn_ptr)(std::hint::black_box(&input));
        }

        let elapsed = start.elapsed();

        iterations = (Duration::from_secs(3).as_nanos() / elapsed.as_nanos()) as u32;

        // Floor iterations to the nearest power of 10
        iterations = 10_u32.pow((iterations as f32).log10().floor() as u32);

        println!("running {} iterations", iterations);

        let now = std::time::Instant::now();

        for _ in 0..iterations {
            (self.fn_ptr)(std::hint::black_box(&input));
        }

        println!("{}: {:?}", self.name, now.elapsed() / iterations);
    }

    pub fn run_and_print(&self, input: &str) -> usize {
        let answer = self.run(input);

        println!("{}: {}", self.name, answer);

        answer
    }
}

#[macro_export]
macro_rules! setup_distributed {
    () => {
        use itertools::Itertools;
        use linkme::distributed_slice;
        use rust_aoc_lib::{PartImplementation, PartNum};

        #[distributed_slice]
        pub static ALL_IMPLEMENTATIONS: [PartImplementation];

        pub fn run_part(part_num: PartNum, input: &str) {
            ALL_IMPLEMENTATIONS
                .iter()
                .sorted_by_key(|p| p.part_num)
                .filter(|p| p.part_num == part_num)
                .map(|p| p.run_and_print(input))
                .reduce(|a, b| {
                    assert_eq!(a, b);
                    a
                });
        }

        pub fn benchmark_part(part_num: PartNum, input: &str) {
            ALL_IMPLEMENTATIONS
                .iter()
                .sorted_by_key(|p| p.part_num)
                .filter(|p| p.part_num == part_num)
                .for_each(|p| p.benchmark(input));
        }

        pub fn do_all(input: &str) {
            run_part(PartNum::Part1, input);
            run_part(PartNum::Part2, input);

            benchmark_part(PartNum::Part1, input);
            benchmark_part(PartNum::Part2, input);
        }
    };
}

pub use linkme;
pub use proc_macros::*;
