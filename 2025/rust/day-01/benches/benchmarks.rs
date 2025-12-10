use day_01::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part_1::process(divan::black_box(include_str!("../data/input.txt"))).unwrap();
}

#[divan::bench]
fn part2() {
    part_2::process(divan::black_box(include_str!("../data/input.txt"))).unwrap();
}