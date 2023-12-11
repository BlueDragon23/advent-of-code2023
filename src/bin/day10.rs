use advent_of_code2023::coordinate::{IndexingCoordinate, PosCoordinate};
use color_eyre::Result;
use itertools::Itertools;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Input {
    map: Vec<Vec<Pipe>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Pipe {
    Start,
    Vertical,
    Horizontal,
    // Pipes joining two directions
    NE,
    NW,
    SE,
    SW,
    Ground,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            'S' => Pipe::Start,
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            'F' => Pipe::SE,
            '7' => Pipe::SW,
            '.' => Pipe::Ground,
            _ => panic!("Unexpected character {}", value),
        }
    }
}

impl Pipe {
    fn get_connected(
        &self,
        coordinate: &IndexingCoordinate,
        max_row: usize,
        max_col: usize,
    ) -> Vec<IndexingCoordinate> {
        // Change type to make going out of bounds temporarily safe
        let (row, col) = (coordinate.row as i32, coordinate.col as i32);
        let max_r = max_row as i32;
        let max_c = max_col as i32;
        match self {
            Pipe::Vertical => vec![(row - 1, col), (row + 1, col)],
            Pipe::Horizontal => vec![(row, col - 1), (row, col + 1)],
            Pipe::NE => vec![(row - 1, col), (row, col + 1)],
            Pipe::NW => vec![(row - 1, col), (row, col - 1)],
            Pipe::SE => vec![(row + 1, col), (row, col + 1)],
            Pipe::SW => vec![(row + 1, col), (row, col - 1)],
            Pipe::Ground => vec![],
            _ => panic!("Invalid pipe, {:?} cannot be connected", self),
        }
        .iter()
        .filter_map(|(r, c)| {
            if *r < 0 || *r >= max_r || *c < 0 || *c >= max_c {
                None
            } else {
                Some(IndexingCoordinate {
                    row: *r as usize,
                    col: *c as usize,
                })
            }
        })
        .collect_vec()
    }
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let map = parsing::parse_input(include_str!("../../input/day10.txt"))?;
    let input = Input { map };
    let time = Instant::now();
    println!(
        "Part 1: {} in {}ms",
        solve_part1(&input),
        time.elapsed().as_millis()
    );
    let time = Instant::now();
    println!(
        "Part 2: {} in {}ms",
        solve_part2(&input),
        time.elapsed().as_millis()
    );
    Ok(())
}

mod parsing {

    use crate::Pipe;

    use color_eyre::Result;
    use nom::character::complete::one_of;
    use nom::error::Error;
    use nom::multi::many1;
    use nom::{combinator::map, Finish, IResult};

    fn parse_line(input: &str) -> IResult<&str, Vec<Pipe>> {
        many1(map(one_of("|-LJ7F.S"), Pipe::from))(input)
    }

    pub fn parse_input(input: &str) -> Result<Vec<Vec<Pipe>>, Error<&str>> {
        input
            .lines()
            .map(|line| parse_line(line).finish().map(|x| x.1))
            .collect()
    }
}

fn find_start(map: &[Vec<Pipe>]) -> IndexingCoordinate {
    map.iter()
        .enumerate()
        .find_map(|(row, pipes)| {
            pipes.iter().enumerate().find_map(|(col, pipe)| {
                if let Pipe::Start = pipe {
                    Some(IndexingCoordinate { row, col })
                } else {
                    None
                }
            })
        })
        .unwrap()
}

fn identify_pipe(start: IndexingCoordinate, connecting_pipes: Vec<IndexingCoordinate>) -> Pipe {
    if connecting_pipes.len() != 2 {
        panic!("Expected exactly two adjacent pipes")
    }
    let (a, b) = (connecting_pipes[0], connecting_pipes[1]);
    // Only one dimension can change
    match start.row.cmp(&a.row) {
        // Below
        std::cmp::Ordering::Less => todo!(),
        std::cmp::Ordering::Equal => match start.col.cmp(&a.col) {
            // Right
            std::cmp::Ordering::Less => todo!(),
            // Left
            std::cmp::Ordering::Greater => todo!(),
            _ => panic!("Forbidden"),
        },
        // Above
        std::cmp::Ordering::Greater => todo!(),
    }
    Pipe::Vertical
}

fn solve_part1(input: &Input) -> u32 {
    let start = find_start(&input.map);
    let max_row = input.map.len();
    let max_col = input.map[0].len();
    // First, find what tile the start actually is
    let start_type = identify_pipe(
        start,
        start
            .get_adjacent_points(max_row, max_col)
            .iter()
            .filter(|c| {
                let p = c.get(&input.map);
                p.get_connected(c, max_row, max_col).contains(&start)
            })
            .map(|x| *x)
            .collect_vec(),
    );
    1
}

fn solve_part2(input: &Input) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = Input {
            map: parsing::parse_input(include_str!("../../input/day10.test.txt"))?,
        };
        let result = solve_part1(&input);
        assert_eq!(result, 4);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = Input {
            map: parsing::parse_input(include_str!("../../input/day10.test.txt"))?,
        };
        let result = solve_part2(&input);
        assert_eq!(result, 1);
        Ok(())
    }
}
