use color_eyre::Result;
use itertools::Itertools;
use num::range;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Input {
    seeds: Vec<u64>,
    mappings: Vec<Vec<MappingRow>>,
}

#[derive(Debug, Clone)]
pub struct MappingRow {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl MappingRow {
    fn source_contains(&self, other: u64) -> bool {
        self.source_start <= other && other <= self.source_start + self.length
    }

    fn get_destination(&self, other: u64) -> u64 {
        self.destination_start + (other - self.source_start)
    }
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day5.txt"))?;
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

    use crate::MappingRow;

    use super::Input;
    use advent_of_code2023::parsing::parse_u64;
    use color_eyre::Result;
    use nom::bytes::complete::take_till;
    use nom::error::Error;
    use nom::multi::separated_list1;
    use nom::sequence::{pair, preceded, separated_pair, terminated, tuple};
    use nom::{bytes::complete::tag, combinator::map, Finish, IResult};

    fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
        preceded(tag("seeds: "), separated_list1(tag(" "), parse_u64))(input)
    }

    fn parse_mapping_row(input: &str) -> IResult<&str, MappingRow> {
        map(
            tuple((
                terminated(parse_u64, tag(" ")),
                terminated(parse_u64, tag(" ")),
                parse_u64,
            )),
            |(destination, source, length)| MappingRow {
                destination_start: destination,
                source_start: source,
                length,
            },
        )(input)
    }

    fn parse_mapping(input: &str) -> IResult<&str, Vec<MappingRow>> {
        // skip a line, then read mappings
        preceded(
            pair(take_till(|c| c == '\n'), tag("\n")),
            separated_list1(tag("\n"), parse_mapping_row),
        )(input)
    }

    pub fn parse_input(input: &str) -> Result<Input, Error<&str>> {
        map(
            separated_pair(
                parse_seeds,
                tag("\n\n"),
                separated_list1(tag("\n\n"), parse_mapping),
            ),
            |(seeds, mappings)| Input { seeds, mappings },
        )(input)
        .finish()
        .map(|r| r.1)
    }
}

fn get_location_for_seed(seed: u64, mappings: &Vec<Vec<MappingRow>>) -> u64 {
    mappings.iter().fold(seed, |item, mapping| {
        mapping
            .iter()
            .find(|map| map.source_contains(item))
            .map(|map| map.get_destination(item))
            .unwrap_or(item)
    })
}

fn solve_part1(input: &Input) -> u64 {
    input
        .seeds
        .iter()
        .map(|seed| get_location_for_seed(*seed, &input.mappings))
        .min()
        .unwrap()
}

fn solve_part2(input: &Input) -> u64 {
    input
        .seeds
        .iter()
        .tuples()
        .flat_map(|(seed_start, seed_length)| {
            range(*seed_start, *seed_start + *seed_length - 1)
                .into_iter()
                .map(|seed| get_location_for_seed(seed, &input.mappings))
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day5.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 35);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day5.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 46);
        Ok(())
    }
}
