use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Range {
    start: String,
    end: String,
}

#[derive(Debug, PartialEq)]
pub enum ParrseRangeError {
    InvalidFormat,
    InvalidNumber,
}

impl FromStr for Range {
    type Err = ParrseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let boundries: Vec<&str> = s.split('-').collect();
        if boundries.len() != 2 {
            return Err(ParrseRangeError::InvalidFormat);
        }

        for boundrie in &boundries {
            let _is_i64: i64 = boundrie
                .parse()
                .map_err(|_| ParrseRangeError::InvalidNumber)?;
        }

        Ok(Range {
            start: boundries[0].to_string(),
            end: boundries[1].to_string(),
        })
    }
}

impl Range {
    fn sum_invalids(&self, part2: bool) -> i64 {
        let mut sum = 0;

        sum += _parse_if_invalid(&self.start, part2);
        sum += _parse_if_invalid(&self.end, part2);

        let start = _parse(&self.start);
        let end = _parse(&self.end);

        for el in (start + 1)..(end - 1) {
            sum += _parse_if_invalid(&el.to_string(), part2);
        }

        sum
    }
}

fn _parse_if_invalid(s: &str, part2: bool) -> i64 {
    if !(is_valid_string_part1(s) && is_valid_string_part2(s, part2)) {
        return _parse(s);
    }

    0
}

fn _parse(s: &str) -> i64 {
    match s.parse().ok() {
        Some(x) => x,
        _ => 0,
    }
}

fn is_valid_string_part1(s: &str) -> bool {
    let (first_half, second_half) = s.split_at(s.len() / 2);

    first_half.len() != second_half.len() || first_half != second_half
}

fn is_valid_string_part2(s: &str, part2: bool) -> bool {
    if !part2 {
        return true;
    }

    for size in 1..=(s.len() / 2) {
        if s.len() % size != 0 {
            continue;
        }
        let is_valid = is_valid_string_part2_size(s, size);
        if !is_valid {
            return false;
        }
    }

    true
}

fn is_valid_string_part2_size(s: &str, size: usize) -> bool {
    let repetitions = s.len() / size;
    let first_word = &s[0..size];
    for i in 1..repetitions {
        let word = &s[size * i..size * (i + 1)];
        if first_word != word {
            return true;
        }
    }
    false
}

fn day02(input: String, part2: bool) -> i64 {
    let ranges: Vec<Range> = aoc::read_from_str_one_liner(&input);
    let mut sum = 0;

    for range in ranges {
        sum += range.sum_invalids(part2);
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();
        assert_eq!(day02(input, false), 1227775554);
    }

    #[test]
    fn puzzle_part1() {
        let input = std::fs::read_to_string("input/day_02.txt").expect("Failled to read file");
        assert_eq!(day02(input, false), 38437576669);
    }

    #[test]
    fn example_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();
        assert_eq!(day02(input, true), 4174379265);
    }

    #[test]
    fn puzzle_part2() {
        let input = std::fs::read_to_string("input/day_02.txt").expect("Failled to read file");
        assert_eq!(day02(input, true), 49046150754);
    }
}
