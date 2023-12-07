use advent_of_code2023::coordinate::{Coordinate, IndexingCoordinate};
use color_eyre::Result;
use itertools::Itertools;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Input {
    elements: Vec<Element>,
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day3.txt"))?;
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

#[derive(Debug, Clone, Copy)]
enum Element {
    PartHead(u32),
    // Store offset from the head
    PartTail(usize),
    Empty,
    Symbol(char),
}

mod parsing {

    use crate::Element;

    use super::Input;
    use advent_of_code2023::parsing::parse_number;
    use color_eyre::Result;
    use itertools::Itertools;
    use nom::branch::alt;
    use nom::character::complete::none_of;
    use nom::error::Error;
    use nom::multi::many1;
    use nom::{bytes::complete::tag, combinator::map, Finish, IResult};
    use num::range;

    fn parse_symbol(input: &str) -> IResult<&str, Vec<Element>> {
        map(none_of("1234567890."), |c| vec![Element::Symbol(c)])(input)
    }

    fn parse_empty(input: &str) -> IResult<&str, Vec<Element>> {
        map(tag("."), |_| vec![Element::Empty])(input)
    }

    fn parse_part(input: &str) -> IResult<&str, Vec<Element>> {
        map(parse_number, |x: u32| {
            range(0, x.to_string().len())
                .map(|i| {
                    if i == 0 {
                        Element::PartHead(x)
                    } else {
                        Element::PartTail(i)
                    }
                })
                .collect_vec()
        })(input)
    }

    fn parse_element(input: &str) -> IResult<&str, Vec<Element>> {
        alt((parse_part, parse_empty, parse_symbol))(input)
    }

    fn parse_line(input: &str) -> IResult<&str, Input> {
        map(many1(parse_element), |elements| Input {
            elements: elements.into_iter().flatten().collect_vec(),
        })(input)
    }

    pub fn parse_input(input: &str) -> Result<Vec<Vec<Element>>, Error<&str>> {
        input
            .lines()
            .map(|line| parse_line(line).finish().map(|x| x.1.elements))
            .collect()
    }
}

fn get_adjacent_parts(
    coord: IndexingCoordinate,
    input: &[Vec<Element>],
    max_row: usize,
    max_col: usize,
) -> Vec<IndexingCoordinate> {
    coord
        .get_adjacent_points_diagonal(max_row, max_col)
        .iter()
        .filter_map(|adj| {
            let other = adj.get(input);
            match other {
                Element::PartHead(_) => Some(*adj),
                Element::PartTail(offset) => {
                    let head = Coordinate::new(adj.row, adj.col - offset);
                    match head.get(input) {
                        Element::PartHead(_) => Some(head),
                        element => panic!("Expected to find PartHead but found {:?}", element),
                    }
                }
                _ => None,
            }
        })
        .unique()
        .collect_vec()
}

fn solve_part1(input: &Vec<Vec<Element>>) -> u32 {
    let max_row = input.len();
    let max_col = input[0].len();
    let parts = input
        .iter()
        .enumerate()
        .flat_map(|(row, elements)| {
            elements
                .iter()
                .enumerate()
                .filter_map(|(col, element)| {
                    if let Element::Symbol(_) = element {
                        let coord = IndexingCoordinate::new(row, col);
                        return Some(get_adjacent_parts(coord, input, max_row, max_col));
                    }
                    None
                })
                .flatten()
                .collect_vec()
        })
        .unique()
        .collect_vec();
    parts
        .iter()
        .map(|c| match c.get(input) {
            Element::PartHead(value) => value,
            other => panic!("Expected PartHead but got {:?}", other),
        })
        .sum()
}

fn solve_part2(input: &Vec<Vec<Element>>) -> u32 {
    let max_row = input.len();
    let max_col = input[0].len();
    input
        .iter()
        .enumerate()
        .map(|(row, elements)| {
            elements
                .iter()
                .enumerate()
                .filter_map(|(col, element)| {
                    if let Element::Symbol('*') = element {
                        let coord = IndexingCoordinate::new(row, col);
                        let adj_parts = get_adjacent_parts(coord, input, max_row, max_col);
                        if adj_parts.len() == 2 {
                            return match (adj_parts[0].get(input), adj_parts[1].get(input)) {
                                (Element::PartHead(a), Element::PartHead(b)) => Some(a * b),
                                _ => panic!("Invalid items found"),
                            };
                        } else {
                            return None;
                        }
                    }
                    None
                })
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day3.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 4361);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day3.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 467835);
        Ok(())
    }
}
