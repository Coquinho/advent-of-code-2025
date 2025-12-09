#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub enum ParsePointError  {
    Format,
    X,
    Y
}

impl std::str::FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = s.split(',').collect();
        if splits.len() != 2 {
            return Err(ParsePointError::Format);
        }
        let x: i32 = splits[0].parse().map_err(|_| ParsePointError::X)?;
        let y: i32 = splits[1].parse().map_err(|_| ParsePointError::Y)?;

        Ok(Point { x, y })
    }
}

fn day09part1(input: String) -> i64 {
    let points: Vec<Point> = aoc::read_from_str(&input);
    println!("{:?}", points);

    let mut max_area = 0;
    for a in &points {
        for b in &points {
            if a == b {
                continue;
            }
            let area = ((a.x - b.x +1) as i64 * (a.y - b.y +1) as i64).abs();
            max_area = max_area.max(area);
        }

    }

    max_area 
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3".to_string();
        assert_eq!(day09part1(input), 50);
    }

    #[test]
    fn puzzle_part1() {
        let input = std::fs::read_to_string("input/day_09.txt").expect("Failled to read file");
        assert_eq!(day09part1(input), 50);
    }
}
