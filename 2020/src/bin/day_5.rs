use aoc_2020::Problem;

#[derive(Debug)]
struct Directions(VerticalDirections, HorizontalDirections);

#[derive(Debug)]
struct VerticalDirections([VerticalDirection; 7]);

impl VerticalDirections {
    fn calculate_row(self) -> u8 {
        self.0
            .into_iter()
            .fold((0u8, 127u8), |acc, vertical_direction| {
                let mid = ((acc.1 - acc.0) / 2) + acc.0;
                match vertical_direction {
                    VerticalDirection::Front => (acc.0, mid),
                    VerticalDirection::Back => (mid + 1, acc.1),
                }
            })
            .0
    }
}

#[derive(Debug)]
struct HorizontalDirections([HorizontalDirection; 3]);

impl HorizontalDirections {
    fn calculate_seat(self) -> u8 {
        self.0
            .into_iter()
            .fold((0u8, 7u8), |acc, horizontal_direction| {
                let mid = ((acc.1 - acc.0) / 2) + acc.0;
                match horizontal_direction {
                    HorizontalDirection::Left => (acc.0, mid),
                    HorizontalDirection::Right => (mid + 1, acc.1),
                }
            })
            .1
    }
}

#[derive(Debug)]
enum HorizontalDirection {
    Left,
    Right,
}

impl TryFrom<char> for HorizontalDirection {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            c => Err(format!(
                "Invalid Character attempted to convert to VerticalDirection: {c}"
            )),
        }
    }
}

#[derive(Debug)]
enum VerticalDirection {
    Front,
    Back,
}

impl TryFrom<char> for VerticalDirection {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'F' => Ok(Self::Front),
            'B' => Ok(Self::Back),
            c => Err(format!(
                "Invalid Character attempted to convert to HorizontalDirection: {c}"
            )),
        }
    }
}

struct Day5Problem;

impl Day5Problem {
    fn parse(input: &str) -> Vec<Directions> {
        input
            .lines()
            .filter_map(|line| {
                let line = line.trim();

                if line.is_empty() {
                    return None;
                };

                let mut char_iter = line.chars();

                let vertical_direction: [VerticalDirection; 7] = [
                    char_iter
                        .next()
                        .map(|char| VerticalDirection::try_from(char).ok())??,
                    char_iter
                        .next()
                        .map(|char| VerticalDirection::try_from(char).ok())??,
                    char_iter
                        .next()
                        .map(|char| VerticalDirection::try_from(char).ok())??,
                    char_iter
                        .next()
                        .map(|char| VerticalDirection::try_from(char).ok())??,
                    char_iter
                        .next()
                        .map(|char| VerticalDirection::try_from(char).ok())??,
                    char_iter
                        .next()
                        .map(|char| VerticalDirection::try_from(char).ok())??,
                    char_iter
                        .next()
                        .map(|char| VerticalDirection::try_from(char).ok())??,
                ];

                let horizontal_direction: [HorizontalDirection; 3] = [
                    char_iter
                        .next()
                        .map(|char| HorizontalDirection::try_from(char).ok())??,
                    char_iter
                        .next()
                        .map(|char| HorizontalDirection::try_from(char).ok())??,
                    char_iter
                        .next()
                        .map(|char| HorizontalDirection::try_from(char).ok())??,
                ];

                Some(Directions(
                    VerticalDirections(vertical_direction),
                    HorizontalDirections(horizontal_direction),
                ))
            })
            .collect()
    }
}

impl Problem for Day5Problem {
    fn part_one(input: &str) -> String {
        let data = Day5Problem::parse(input);
        data.into_iter()
            .map(|Directions(vertical_direction, horizontal_direction)| {
                let row = vertical_direction.calculate_row();
                let seat = horizontal_direction.calculate_seat();

                row as u32 * 8u32 + seat as u32
            })
            .max()
            .unwrap()
            .to_string()
    }

    fn part_two(input: &str) -> String {
        let data = Day5Problem::parse(input);
        let mut seat_ids: Vec<u32> = data
            .into_iter()
            .map(|Directions(vertical_direction, horizontal_direction)| {
                let row = vertical_direction.calculate_row();
                let seat = horizontal_direction.calculate_seat();

                row as u32 * 8u32 + seat as u32
            })
            .collect();
        {
            seat_ids.sort();
        }

        let results: Vec<u32> = (seat_ids.first().unwrap().clone()
            ..seat_ids.last().unwrap().clone())
            .into_iter()
            .filter(|seat_id| {
                !seat_ids.contains(seat_id)
                    && seat_ids.contains(&(*seat_id - 1))
                    && seat_ids.contains(&(*seat_id + 1))
            })
            .collect();

        results.first().unwrap().to_string()
    }
}

fn main() {
    println!("{:?}", std::env::current_dir());
    let file = std::fs::read("./aoc_2020/data/day_5.txt").unwrap();
    let input = std::str::from_utf8(&file).unwrap();

    println!("Part 1: {}", Day5Problem::part_one(input));
    println!("Part 2: {}", Day5Problem::part_two(input));
}

#[cfg(test)]
mod tests {
    use aoc_2020::Problem;

    use crate::Day5Problem;

    #[test]
    fn part_1() {
        assert_eq!("357", Day5Problem::part_one("FBFBBFFRLR"));
        assert_eq!("567", Day5Problem::part_one("BFFFBBFRRR"));
        assert_eq!("119", Day5Problem::part_one("FFFBBBFRRR"));
        assert_eq!("820", Day5Problem::part_one("BBFFBBFRLL"));
    }

    #[test]
    fn part_2() {
        assert_eq!("1", Day5Problem::part_two(")"));
        assert_eq!("5", Day5Problem::part_two("()())"));
    }
}
