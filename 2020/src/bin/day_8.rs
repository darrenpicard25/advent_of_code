use aoc_2020::Problem;

struct Day8Problem;

impl Problem for Day8Problem {
    fn part_one(input: &str) -> String {
        "todo".into()
    }

    fn part_two(input: &str) -> String {
        "todo".into()
    }
}

fn main() {
    println!("{:?}", std::env::current_dir());
    let file = std::fs::read("./aoc_2020/data/day_8.txt").unwrap();
    let input = std::str::from_utf8(&file).unwrap();

    println!("Part 1: {}", Day8Problem::part_one(input));
    println!("Part 2: {}", Day8Problem::part_two(input));
}

#[cfg(test)]
mod tests {
    use aoc_2020::Problem;

    use crate::Day8Problem;

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
        assert_eq!("4", Day8Problem::part_one(INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!("6", Day8Problem::part_two(INPUT));
    }
}
