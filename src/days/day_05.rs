use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct FreshRange {
    start: i64,
    end: i64,
}

#[derive(Debug, PartialEq)]
pub enum ParseFreshRangeError {
    InvalidStart,
    InvalidEnd,
    InvalidFormat,
}

impl FromStr for FreshRange {
    type Err = ParseFreshRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let boundaries: Vec<&str> = s.split('-').collect();
        if boundaries.len() != 2 {
            return Err(ParseFreshRangeError::InvalidFormat);
        }
        let start = boundaries[0]
            .parse()
            .map_err(|_| ParseFreshRangeError::InvalidStart)?;
        let end = boundaries[1]
            .parse()
            .map_err(|_| ParseFreshRangeError::InvalidEnd)?;

        Ok(FreshRange { start, end })
    }
}

impl PartialOrd<i64> for FreshRange {
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        if *other < self.start {
            return Some(Ordering::Less);
        } else if *other > self.end {
            return Some(Ordering::Greater);
        } else {
            return Some(Ordering::Equal);
        }
    }
}

impl PartialEq<i64> for FreshRange {
    fn eq(&self, other: &i64) -> bool {
        self.start <= *other && self.end >= *other
    }
}

#[derive(Debug, PartialEq)]
struct Input {
    fresh_ranges: Vec<FreshRange>,
    ingredients: Vec<i64>,
}

#[derive(Debug, PartialEq)]
pub enum ParseInputError {
    InvalidFormat,
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s.split("\n\n").collect();
        if input.len() != 2 {
            return Err(ParseInputError::InvalidFormat);
        }
        let fresh_ranges: Vec<FreshRange> = aoc::read_from_str(&input[0]);

        let ingredients: Vec<i64> = aoc::read_from_str(&input[1]);

        Ok(Input {
            fresh_ranges,
            ingredients,
        })
    }
}

fn optimize_fresh_ranges(fresh_ranges: &mut Vec<FreshRange>) -> Vec<FreshRange> {
    fresh_ranges.sort();
    let mut new_fresh_ranges: Vec<FreshRange> = Vec::new();

    let mut new = FreshRange { ..fresh_ranges[0] };
    for current in fresh_ranges {
        if new.end >= current.end {
            continue;
        }
        if new.end >= current.start {
            new.end = current.end;
            continue;
        }
        new_fresh_ranges.push(new);
        new = FreshRange { ..*current };
    }
    new_fresh_ranges.push(new);

    new_fresh_ranges
}
fn day05part1(input: String) -> (i64, i64) {
    let mut input: Input = input.parse().expect("Error parsing input");
    let fresh_ranges: Vec<FreshRange> = optimize_fresh_ranges(&mut input.fresh_ranges);
    let mut ingredients: Vec<i64> = input.ingredients;
    ingredients.sort();

    let mut ans: (i64, i64) = (0, 0);

    let mut fresh_range_iter = fresh_ranges.iter();
    let mut fresh_range = fresh_ranges[0];
    for ingredient in &ingredients {
        while fresh_range > *ingredient {
            if let Some(new_range) = fresh_range_iter.next() {
                fresh_range = *new_range;
                ans.1 += fresh_range.end - fresh_range.start + 1;
            } else {
                break;
            }
        }
        if fresh_range == *ingredient {
            ans.0 += 1;
        }
    }
    while let Some(fresh_range) = fresh_range_iter.next() {
        ans.1 += fresh_range.end - fresh_range.start + 1;
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32".to_string();
        assert_eq!(day05part1(input), (3, 14));
    }

    #[test]
    fn puzzle_part1() {
        let input = std::fs::read_to_string("input/day_05.txt").expect("Failled to read file");
        assert_eq!(day05part1(input), (811, 338189277144473));
    }
}
