use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Rotation {
    AntiClockwise(i32),
    Clockwise(i32),
}

#[derive(Debug, PartialEq)]
pub enum ParseRotationError {
    InvalidFormat,
    InvalidDirection,
    InvalidNumber,
}

impl FromStr for Rotation {
    type Err = ParseRotationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(ParseRotationError::InvalidFormat);
        }

        let mut chars = s.chars();
        let direction = chars.next().unwrap();
        let number: String = chars.collect();

        let number: i32 = number
            .parse()
            .map_err(|_| ParseRotationError::InvalidNumber)?;

        match direction {
            'L' => Ok(Rotation::AntiClockwise(number)),
            'R' => Ok(Rotation::Clockwise(number)),
            _ => Err(ParseRotationError::InvalidDirection),
        }
    }
}

fn step(start: i32, rotation: &Rotation) -> (i32, i32) {
    const MODULUS: i32 = 100;
    match rotation {
        Rotation::AntiClockwise(x) => {
            let mut end = start - x;
            let mut full_rotations = (end / MODULUS).abs();
            end = end % MODULUS;
            if end < 0 {
                end += MODULUS;
                if start > 0 {
                    full_rotations += 1;
                }
            }
            (end, full_rotations)
        }
        Rotation::Clockwise(x) => {
            let mut end = start + x;
            let mut full_rotations = end / MODULUS;
            end = end % MODULUS;

            if end == 0 {
                full_rotations -= 1;
            }
            (end, full_rotations)
        }
    }
}

fn day01(input: String, should_count_rotations: bool) -> i32 {
    let rotations = aoc::read_from_str(&input);

    let mut current_position: i32 = 50;
    let mut count = 0;

    for rotation in &rotations {
        let old_position = current_position;
        let full_rotations: i32;
        (current_position, full_rotations) = step(current_position, rotation);
        if current_position == 0 {
            count += 1;
        }
        if should_count_rotations {
            count += full_rotations;
        }
        println!(
            "current {:?} {:?} new {:?} pass {:?} count {:?}",
            old_position, rotation, current_position, full_rotations, count
        );
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "\nL68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string();
        assert_eq!(day01(input, false), 3);
    }

    #[test]
    fn puzzle_part1() {
        let input = std::fs::read_to_string("input/day_01.txt")
            .expect("Failled to read file");
        assert_eq!(day01(input, false), 1191);
    }

    #[test]
    fn example_part2() {
        let input = "\nL68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string();
        assert_eq!(day01(input, true), 6);
    }

    #[test]
    fn anti_clockwise() {
        let input = "\nL750".to_string();
        assert_eq!(day01(input, false), 1);
        let input = "\nL268".to_string();
        assert_eq!(day01(input, false), 0);
    }

    #[test]
    fn clockwise() {
        let input = "\nR750".to_string();
        assert_eq!(day01(input, false), 1);
        let input = "\nL268".to_string();
        assert_eq!(day01(input, false), 0);
    }
    #[test]
    fn anti_clockwise_with_pass_throug() {
        let input = "\nL750".to_string();
        assert_eq!(day01(input, true), 8);
        let input = "\nL268".to_string();
        assert_eq!(day01(input, true), 3);
    }

    #[test]
    fn clockwise_with_pass_throug() {
        let input = "\nR750".to_string();
        assert_eq!(day01(input, true), 8);
        let input = "\nR268".to_string();
        assert_eq!(day01(input, true), 3);
    }

    #[test]
    fn puzzle_part2() {
        let input = std::fs::read_to_string("input/day_01.txt")
            .expect("Failled to read file");
        assert_eq!(day01(input, true), 6858);
    }
}
