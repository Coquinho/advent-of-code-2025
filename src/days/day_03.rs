use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Bank {
    bateries: Vec<i8>,
}

#[derive(Debug, PartialEq)]
pub enum ParseBankError {
    InvalidBankSize,
    InvalidBatery,
}

impl FromStr for Bank {
    type Err = ParseBankError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(ParseBankError::InvalidBankSize);
        }

        let bateries: Vec<i8> = s
            .chars()
            .into_iter()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as i8)
            .collect();

        Ok(Bank { bateries: bateries })
    }
}

fn day03(input: String) -> i32 {
    let banks: Vec<Bank> = aoc::read_from_str(&input);
    let mut sum = 0;

    println!("{:?}", banks);
    for bank in banks {
        let bateries = &bank.bateries;
        let mut first = bateries[0];
        let mut second = bateries[1];
        for i in 1..bateries.len() {
            let batery = bateries[i];

            if batery > first && i < bateries.len() - 1 {
                first = batery;
                second = bateries[i + 1];
                continue;
            } else if batery > second {
                second = batery;
            }
        }
        let joltage = (first as i32) * 10 + (second as i32);
        sum += joltage;
        println!("{:?} {:?} {:?} {:?}", sum, joltage, first, second);
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part1() {
        let input =
            "987654321111111\n811111111111119\n234234234234278\n818181911112111".to_string();
        assert_eq!(day03(input), 357);
    }

    #[test]
    fn puzzle_part1() {
        let input = std::fs::read_to_string("input/day_03.txt").expect("Failled to read file");
        assert_eq!(day03(input), 16812);
    }
}
