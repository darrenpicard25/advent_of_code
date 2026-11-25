use day_01 as day;
use divan::black_box;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let _ = day::part1(black_box(day::INPUT)).unwrap();
}

#[divan::bench]
fn part2() {
    let _ = day::part2(black_box(day::INPUT)).unwrap();
}

#[divan::bench]
fn both() {
    let input = black_box(day::INPUT);
    let _ = day::part1(input).unwrap();
    let _ = day::part2(input).unwrap();
}
