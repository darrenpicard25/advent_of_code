use {{crate_name}}::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part_1::process(divan::black_box(include_str!(
        "../data/part_1_input.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part2() {
    part_2::process(divan::black_box(include_str!(
        "../data/part_2_input.txt",
    )))
    .unwrap();
}