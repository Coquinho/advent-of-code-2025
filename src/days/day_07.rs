use std::collections::{HashMap, HashSet};

type Beam = (usize, usize);

#[derive(Debug, PartialEq)]
struct Input {
    beams: HashMap<Beam, usize>,
    splitters: HashMap<usize, HashSet<usize>>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
pub enum ParseInputError {
    Width,
    NoBeam,
    Char,
}

impl std::str::FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const BEAM: char = 'S';
        const EMPTY: char = '.';
        const SPLITTER: char = '^';

        let mut beams: HashMap<Beam, usize> = HashMap::new();
        let mut splitters: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut height = 0;
        let mut width = 0;

        let lines: Vec<&str> = s.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            let chars: Vec<char> = line.trim().chars().collect();
            if width == 0 {
                width = chars.len();
            } else if width != chars.len() {
                return Err(ParseInputError::Width);
            }
            height += 1;

            for (j, c) in chars.iter().enumerate() {
                match *c {
                    EMPTY => (),
                    BEAM => {
                        beams.insert((i, j), 1);
                    }
                    SPLITTER => {
                        let i_splitters = splitters.entry(i).or_default();
                        i_splitters.insert(j);
                    }
                    _ => return Err(ParseInputError::Char),
                };
            }
        }
        if beams.is_empty() {
            return Err(ParseInputError::NoBeam);
        }

        Ok(Input {
            beams,
            splitters,
            width,
            height,
        })
    }
}

#[derive(Clone, Copy)]
enum Mode {
    Classic,
    Quantum,
}

fn add_or_update_beam(
    new_beams: &mut HashMap<Beam, usize>,
    beam: Beam,
    beams_count: usize,
    mode: Mode,
) {
    new_beams
        .entry(beam)
        .and_modify(|count| match mode {
            Mode::Classic => *count = 1,
            Mode::Quantum => *count += beams_count,
        })
        .or_insert(beams_count);
}

impl Input {
    fn process_beam(&mut self, mode: Mode) -> usize {
        let mut splits = 0;
        let mut new_beams: HashMap<Beam, usize> = HashMap::new();

        for (beam, beams_count) in self.beams.iter() {
            if beam.0 >= self.height {
                return splits;
            }
            let new_i = beam.0 + 1;
            let should_split = self
                .splitters
                .get(&new_i)
                .is_some_and(|row_splitters| row_splitters.contains(&beam.1));

            if !should_split {
                add_or_update_beam(&mut new_beams, (new_i, beam.1), *beams_count, mode);
                continue;
            }

            add_or_update_beam(&mut new_beams, (new_i, beam.1 - 1), *beams_count, mode);
            add_or_update_beam(&mut new_beams, (new_i, beam.1 + 1), *beams_count, mode);
            splits += *beams_count;
        }

        self.beams = new_beams;
        splits
    }
}

fn day07(input: String, mode: Mode) -> usize {
    let mut input: Input = input.parse().expect("Error parsing input");
    let mut splits = match mode {
        Mode::Classic => 0,
        Mode::Quantum => 1,
    };

    for _ in 0..input.height {
        splits += input.process_beam(mode);
    }

    splits
}

fn day07part1(input: String) -> usize {
    day07(input, Mode::Classic)
}

fn day07part2(input: String) -> usize {
    day07(input, Mode::Quantum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part1() {
        let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............".to_string();
        assert_eq!(day07part1(input), 21);
    }

    #[test]
    fn puzzle_part1() {
        let input = std::fs::read_to_string("input/day_07.txt").expect("Failled to read file");
        assert_eq!(day07part1(input), 1667);
    }

    #[test]
    fn example_part2() {
        let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............".to_string();
        assert_eq!(day07part2(input), 40);
    }

    #[test]
    fn puzzle_part2() {
        let input = std::fs::read_to_string("input/day_07.txt").expect("Failled to read file");
        assert_eq!(day07part2(input), 62943905501815);
    }
}
