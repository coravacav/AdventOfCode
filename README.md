# Advent of Code Solutions

Welcome to my Advent of Code solutions repository. This repository contains my solutions for the Advent of Code challenges.

## Why is it so big?

I use a few custom proc macros and just normal macros to make all solutions _a little bit_ cleaner.

Basically, any function annotated with `#[part1]` or `#[part2]` is automatically linked up to have proper testing, benchmarking, and input loading.

Whenever you run any bin, it automatically prints results then runs a few benchmarks to be able to compare solutions.
