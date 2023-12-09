use color_eyre::Result;
use itertools::Itertools;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Input {
    history: Vec<i64>,
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day9.txt"))?;
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

    use super::Input;
    use advent_of_code2023::parsing::parse_numbers;
    use color_eyre::Result;
    use nom::combinator::map;
    use nom::error::Error;
    use nom::{Finish, IResult};

    fn parse_line(input: &str) -> IResult<&str, Input> {
        map(parse_numbers, |xs| Input { history: xs })(input)
    }

    pub fn parse_input(input: &str) -> Result<Vec<Input>, Error<&str>> {
        input
            .lines()
            .map(|line| parse_line(line).finish().map(|x| x.1))
            .collect()
    }
}

fn find_differences(numbers: &[i64]) -> Vec<i64> {
    numbers
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec()
}

fn find_next_number(numbers: &[i64]) -> i64 {
    if numbers.iter().all(|x| *x == 0) {
        0
    } else {
        let next_sequence = &find_differences(numbers);
        let next = find_next_number(next_sequence);
        numbers.last().unwrap() + next
    }
}

fn solve_part1(input: &[Input]) -> i64 {
    input.iter().map(|i| find_next_number(&i.history)).sum()
}

fn find_previous_number(numbers: &[i64]) -> i64 {
    if numbers.iter().all(|x| *x == 0) {
        0
    } else {
        let next_sequence = &find_differences(numbers);
        let next = find_previous_number(next_sequence);
        numbers.first().unwrap() - next
    }
}

fn solve_part2(input: &[Input]) -> i64 {
    input.iter().map(|i| find_previous_number(&i.history)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day9.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 114);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day9.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 2);
        Ok(())
    }
}
