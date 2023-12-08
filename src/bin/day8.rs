use color_eyre::Result;
use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

#[derive(Debug, Clone)]
pub struct Input {
    directions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

#[derive(Debug, Clone)]
pub enum Direction {
    L,
    R,
}

#[derive(Debug, Clone)]
pub struct Node {
    left: String,
    right: String,
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day8.txt"))?;
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

    use std::collections::HashMap;

    use crate::{Direction, Node};

    use super::Input;
    use color_eyre::Result;
    use nom::branch::alt;
    use nom::bytes::complete::take;
    use nom::character::complete::char;
    use nom::error::Error;
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{delimited, separated_pair};
    use nom::{bytes::complete::tag, combinator::map, Finish, IResult};

    fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
        many1(map(alt((char('L'), char('R'))), |c| match c {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!("Found unexpected item {}", c),
        }))(input)
    }

    fn parse_node(input: &str) -> IResult<&str, (String, Node)> {
        map(
            separated_pair(
                take(3usize),
                tag(" = "),
                delimited(
                    char('('),
                    separated_pair(take(3usize), tag(", "), take(3usize)),
                    char(')'),
                ),
            ),
            |(name, (left, right)): (&str, (&str, &str))| {
                (
                    name.to_string(),
                    Node {
                        left: left.to_string(),
                        right: right.to_string(),
                    },
                )
            },
        )(input)
    }

    fn parse_nodes(input: &str) -> IResult<&str, HashMap<String, Node>> {
        map(separated_list1(tag("\n"), parse_node), |pairs| {
            pairs.into_iter().collect()
        })(input)
    }

    pub fn parse_input(input: &str) -> Result<Input, Error<&str>> {
        map(
            separated_pair(parse_directions, tag("\n\n"), parse_nodes),
            |(directions, nodes)| Input { directions, nodes },
        )(input)
        .finish()
        .map(|r| r.1)
    }
}

fn solve_part1(input: &Input) -> u32 {
    let mut current = "AAA";
    let mut count = 0;
    for d in input.directions.iter().cycle() {
        let choice = input.nodes.get(current).unwrap();
        let next = match d {
            Direction::L => &choice.left,
            Direction::R => &choice.right,
        };
        count += 1;
        if next == "ZZZ" {
            break;
        }
        current = next;
    }
    count
}

fn solve_part2(input: &Input) -> u32 {
    // To do this efficiently requires cycle detection.
    // Find a number of steps at which each element goes from Z -> Z, then do LCM to find the result
    // In the example, the first element after 'A' is the cycle start, but not sure if that can be assumed
    let mut current = input
        .nodes
        .keys()
        .filter(|name| name.ends_with('A'))
        .collect_vec();
    let mut count = 0;
    for d in input.directions.iter().cycle() {
        let next = current
            .iter()
            .map(|&n| {
                let choice = input.nodes.get(n).unwrap();
                match d {
                    Direction::L => &choice.left,
                    Direction::R => &choice.right,
                }
            })
            .collect_vec();
        count += 1;
        if next.iter().all(|n| n.ends_with('Z')) {
            break;
        }
        current = next;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day8.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 6);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day8.test2.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 6);
        Ok(())
    }
}
