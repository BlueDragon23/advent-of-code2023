use advent_of_code2023::{coordinate::Coordinate, transpose};
use color_eyre::Result;
use itertools::Itertools;
use std::{time::Instant, vec};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Point {
    Galaxy,
    Space,
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day11.txt"))?;
    let time = Instant::now();
    println!(
        "Part 1: {} in {}ms",
        solve_part1(&input),
        time.elapsed().as_millis()
    );
    let time = Instant::now();
    println!(
        "Part 2: {} in {}ms",
        solve_part2(&input, 1_000_000),
        time.elapsed().as_millis()
    );
    Ok(())
}

mod parsing {

    use color_eyre::Result;
    use nom::character::complete::one_of;
    use nom::error::Error;
    use nom::multi::many1;
    use nom::{combinator::map, Finish, IResult};

    use crate::Point;

    fn parse_line(input: &str) -> IResult<&str, Vec<Point>> {
        many1(map(one_of(".#"), |c| {
            if c == '#' {
                Point::Galaxy
            } else {
                Point::Space
            }
        }))(input)
    }

    pub fn parse_input(input: &str) -> Result<Vec<Vec<Point>>, Error<&str>> {
        input
            .lines()
            .map(|line| parse_line(line).finish().map(|x| x.1))
            .collect()
    }
}

fn find_empty_rows(input: &[Vec<Point>]) -> Vec<usize> {
    input
        .iter()
        .enumerate()
        .filter(|(_, points)| points.iter().all(|p| *p == Point::Space))
        .map(|(row, _)| row)
        .collect_vec()
}

fn find_empty_cols(input: &[Vec<Point>]) -> Vec<usize> {
    transpose(input)
        .iter()
        .enumerate()
        .filter(|(_, points)| points.iter().all(|p| *p == Point::Space))
        .map(|(row, _)| row)
        .collect_vec()
}

fn expand(input: &[Vec<Point>]) -> Vec<Vec<Point>> {
    // find empty rows/cols
    let empty_rows = find_empty_rows(input);
    let empty_cols = find_empty_cols(input);
    // Each one of the rows/cols needs to have a _second_ empty row/col added adjacent to it
    input
        .iter()
        .enumerate()
        .flat_map(|(row, points)| {
            let new_points = points
                .iter()
                .enumerate()
                .flat_map(|(col, p)| {
                    if empty_cols.contains(&col) {
                        // add another empty points
                        vec![*p, *p]
                    } else {
                        vec![*p]
                    }
                })
                .collect_vec();
            if empty_rows.contains(&row) {
                vec![new_points.clone(), new_points]
            } else {
                vec![new_points]
            }
        })
        .collect_vec()
}

fn get_galaxy_coordinates(expanded: &[Vec<Point>]) -> Vec<Coordinate<i32>> {
    expanded
        .iter()
        .enumerate()
        .flat_map(|(row, points)| {
            points.iter().enumerate().filter_map(move |(col, p)| {
                if *p == Point::Galaxy {
                    Some((row as i32, col as i32).into())
                } else {
                    None
                }
            })
        })
        .collect_vec()
}

fn solve_part1(input: &[Vec<Point>]) -> i32 {
    let expanded = expand(input);
    if !expanded.iter().map(|r| r.len()).all_equal() {
        panic!("Found invalid galaxy")
    }
    let galaxy_coordinates = get_galaxy_coordinates(&expanded);
    galaxy_coordinates
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.manhattan_distance(b))
        .sum()
}

fn row_between(row: i32, a: &Coordinate<i32>, b: &Coordinate<i32>) -> bool {
    row > a.row && row < b.row || row > b.row && row < a.row
}

fn col_between(col: i32, a: &Coordinate<i32>, b: &Coordinate<i32>) -> bool {
    col > a.col && col < b.col || col > b.col && col < a.col
}

fn solve_part2(input: &[Vec<Point>], expansion: u64) -> u64 {
    let empty_rows = find_empty_rows(input);
    let empty_cols = find_empty_cols(input);
    // When calculating manhattan distance, add expansion * the rows and cols between the start and end
    let galaxy_coordinates = get_galaxy_coordinates(input);
    galaxy_coordinates
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let base = a.manhattan_distance(b) as u64;
            let empty_crossed = empty_cols
                .iter()
                .filter(|&c| col_between(*c as i32, a, b))
                .chain(empty_rows.iter().filter(|&r| row_between(*r as i32, a, b)))
                .count() as u64;
            // println!(
            //     "Distance from a: {:?} to b: {:?} is {}",
            //     a,
            //     b,
            //     base + (empty_crossed * expansion)
            // );
            base + (empty_crossed * (expansion - 1))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day11.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 374);
        Ok(())
    }

    #[test]
    fn test_part1_alt() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day11.test.txt"))?;
        let result = solve_part2(&input, 2);
        assert_eq!(result, 374);
        Ok(())
    }

    #[test]
    fn test_part2_small() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day11.test.txt"))?;
        let result = solve_part2(&input, 10);
        assert_eq!(result, 1030);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day11.test.txt"))?;
        let result = solve_part2(&input, 100);
        assert_eq!(result, 8410);
        Ok(())
    }
}
