use crate::error::AocError;

pub mod error;

pub mod part_1;
pub mod part_2;

const MAX_NUM: u16 = 100_u16;

#[derive(Debug)]
struct DialPosition(u16);

impl DialPosition {
    pub fn new() -> Self {
        Self(50)
    }

    pub fn is_pointing_at_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn move_right(&mut self, amount: u16) -> u16 {
        let mut count = 0;

        for _ in 0..amount {
            if self.0 >= MAX_NUM - 1 {
                self.0 = 0;
                count += 1;
            } else {
                self.0 += 1;
            }
        }

        count
    }

    pub fn move_left(&mut self, amount: u16) -> u16 {
        let mut count = 0;

        for _ in 0..amount {
            if self.0 == 0 {
                self.0 = MAX_NUM;
            }

            self.0 -= 1;

            if self.0 == 0 {
                count += 1;
            }
        }

        count
    }
}

#[derive(Debug)]
enum Instruction {
    Left(u16),
    Right(u16),
}

impl TryFrom<&str> for Instruction {
    type Error = AocError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (direction, amount_str) = value.trim().split_at(1);

        let amount = amount_str.parse::<u16>().map_err(|error| {
            println!(
                "An error occurred parsing amount: {:?}. Value: {}",
                error, amount_str
            );

            AocError::IoError
        })?;

        match direction {
            "L" => Ok(Self::Left(amount)),
            "R" => Ok(Self::Right(amount)),
            _ => {
                eprintln!("Unknown direction when parsing: {}", direction);

                Err(AocError::IoError)
            }
        }
    }
}
