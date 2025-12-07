#[derive(Debug, PartialEq)]
enum Operation{
    Sum(Vec<i64>),
    Mul(Vec<i64>)
}

#[derive(Debug, PartialEq)]
struct Input(Vec<Operation>);

#[derive(Debug, PartialEq)]
pub enum InputParseError {
    InvalidNumber,
    InvalidOperation,
    InvalidFormat
}

impl std::str:FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input: Input = Vec::new();

        let input_str: Vec<Vec<str>> = s
        let operation_size = input_str.len() - 1;

        for i in 0..operation_size {
            line = input_str[i];
            for
        }
    }
}

fn part1(input: String) -> i64 {
    let mut sum = 0;

    sum += 1
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +".to_string();
        assert_eq!(part1(input), 33210);
    }
}
