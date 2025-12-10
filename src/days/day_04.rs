use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
struct Board(Vec<Vec<char>>);

#[derive(Debug, PartialEq)]
pub enum ParseBoardError {
    InvalidChar,
    InvalidWidth,
}

impl FromStr for Board {
    type Err = ParseBoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let mut width = 0;
        let mut board: Vec<Vec<char>> = Vec::new();
        const NON_BLOCK: char = '.';
        let allowed_chars: Vec<char> = ['@', '.'].to_vec();

        for line in lines {
            if width == 0 {
                width = line.len() + 2;
                board.push(vec![NON_BLOCK; width]);
            } else if line.len() != width - 2 {
                return Err(ParseBoardError::InvalidWidth);
            }

            let mut board_line: Vec<char> = vec![NON_BLOCK];
            for c in line.chars() {
                if !allowed_chars.contains(&c) {
                    return Err(ParseBoardError::InvalidChar);
                }
                board_line.push(c);
            }
            board_line.push(NON_BLOCK);
            board.push(board_line);
        }
        board.push(vec![NON_BLOCK; width]);

        Ok(Board(board))
    }
}

fn can_move(board: &Board, i: usize, j: usize) -> bool {
    let mut neighboors_count = 0;

    for n_i in (i - 1)..=(i + 1) {
        for n_j in (j - 1)..=(j + 1) {
            if n_i == i && n_j == j {
                continue;
            }
            if board.0[n_i][n_j] == '@' {
                neighboors_count += 1;
            }

            if neighboors_count >= 4 {
                return false;
            }
        }
    }

    neighboors_count < 4
}

#[derive(PartialEq)]
enum Mode {
    Infinity,
    Once,
}

fn day04(input: String, mode: Mode) -> i32 {
    let mut movable_rolls = 0;

    let mut board: Board = Board::from_str(&input).expect("Error parsing board");
    let mut try_move = true;

    while try_move {
        try_move = false;
        for i in 1..(board.0.len() - 1) {
            let line = &board.0[i].clone();
            for j in 1..(line.len() - 1) {
                if line[j] != '@' {
                    continue;
                }
                let less_than_for_neighboors = can_move(&board, i, j);
                if less_than_for_neighboors {
                    if mode == Mode::Infinity {
                        board.0[i][j] = '.';
                        try_move = true;
                    }
                    movable_rolls += 1;
                }
            }
        }
    }

    movable_rolls
}

fn day04part1(input: String) -> i32 {
    day04(input, Mode::Once)
}

fn day04part2(input: String) -> i32 {
    day04(input, Mode::Infinity)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.".to_string();
        assert_eq!(day04part1(input), 13);
    }

    #[test]
    fn puzzle_part1() {
        let input = std::fs::read_to_string("input/day_04.txt").expect("Failled to read file");
        assert_eq!(day04part1(input), 1547);
    }

    #[test]
    fn example_part2() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.".to_string();
        assert_eq!(day04part2(input), 43);
    }

    #[test]
    fn puzzle_part2() {
        let input = std::fs::read_to_string("input/day_04.txt").expect("Failled to read file");
        assert_eq!(day04part2(input), 8948);
    }
}
