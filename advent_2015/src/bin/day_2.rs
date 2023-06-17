use std::cmp::min;

use advent_2015::Problem;

struct Day2Problem;
// 2*l*w + 2*w*h + 2*h*l

impl Problem for Day2Problem {
    fn part_one(input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let mut items = line.split('x');
                let height = items.next().unwrap().parse::<u32>().unwrap();
                let width = items.next().unwrap().parse::<u32>().unwrap();
                let length = items.next().unwrap().parse::<u32>().unwrap();

                let side_1 = length * width;
                let side_2 = height * length;
                let side_3 = width * height;

                2 * side_1 + 2 * side_2 + 2 * side_3 + min(side_1, min(side_2, side_3))
            })
            .sum::<u32>()
            .to_string()
    }

    fn part_two(input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let mut items = line.split('x');
                let height = items.next().unwrap().parse::<u32>().unwrap();
                let width = items.next().unwrap().parse::<u32>().unwrap();
                let length = items.next().unwrap().parse::<u32>().unwrap();

                let smallest_length_1 = min(height, width);

                let smallest_length_2 = if smallest_length_1 == height {
                    min(width, length)
                } else {
                    min(height, length)
                };

                (2 * smallest_length_1) + (2 * smallest_length_2) + (height * length * width)
            })
            .sum::<u32>()
            .to_string()
    }
}

fn main() {
    println!("{:?}", std::env::current_dir());
    let file = std::fs::read("./advent_2015/data/day_2.txt").unwrap();
    let input = std::str::from_utf8(&file).unwrap();

    println!("Part 1: {}", Day2Problem::part_one(input));
    println!("Part 2: {}", Day2Problem::part_two(input));
}

#[cfg(test)]
mod tests {
    use advent_2015::Problem;

    use crate::Day2Problem;

    #[test]
    fn part_1() {
        assert_eq!("58", Day2Problem::part_one("2x3x4"));
        assert_eq!("43", Day2Problem::part_one("1x1x10"));
    }

    #[test]
    fn part_2() {
        assert_eq!("34", Day2Problem::part_two("2x3x4"));
        assert_eq!("14", Day2Problem::part_two("1x1x10"));
    }
}
