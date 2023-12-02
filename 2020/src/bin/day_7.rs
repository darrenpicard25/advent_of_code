use aoc_2020::Problem;

struct Day7Problem;

impl Day7Problem {
    fn parse(input: &'static str) -> () {
        let x = input.lines().filter_map(|line| {
            let line = line.trim();

            if line.is_empty() {
                return None;
            }

            let mut split = line.split("bags contain");

            let first = split.next().unwrap();
            let second = split.next().unwrap();

            Some(line)
        });

        todo!()
    }
}

impl Problem for Day7Problem {
    fn part_one(input: &str) -> String {
        "todo".into()
    }

    fn part_two(input: &str) -> String {
        "todo".into()
    }
}

fn main() {
    println!("{:?}", std::env::current_dir());
    let file = std::fs::read("./aoc_2020/data/day_6.txt").unwrap();
    let input = std::str::from_utf8(&file).unwrap();

    println!("Part 1: {}", Day7Problem::part_one(input));
    println!("Part 2: {}", Day7Problem::part_two(input));
}

#[cfg(test)]
mod tests {
    use aoc_2020::Problem;

    use crate::Day7Problem;

    const INPUT: &'static str = r#"
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;
    #[test]
    fn part_1() {
        assert_eq!("4", Day7Problem::part_one(INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!("6", Day7Problem::part_two(INPUT));
    }
}
