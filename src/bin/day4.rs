use color_eyre::Result;
use num::range;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Input {
    winning: Vec<u32>,
    our_numbers: Vec<u32>,
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day4.txt"))?;
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
    use color_eyre::Result;
    use nom::bytes::complete::{take, take_until};
    use nom::combinator::map_opt;
    use nom::error::Error;
    use nom::multi::separated_list1;
    use nom::sequence::{pair, preceded, separated_pair};
    use nom::{bytes::complete::tag, combinator::map, Finish, IResult};

    fn parse_number(input: &str) -> IResult<&str, u32> {
        // each number is always 2 digits
        map_opt(take(2usize), |x: &str| x.trim().parse::<u32>().ok())(input)
    }

    fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
        separated_list1(tag(" "), parse_number)(input)
    }

    fn parse_line(input: &str) -> IResult<&str, Input> {
        map(
            preceded(
                pair(take_until(": "), tag(": ")),
                separated_pair(parse_numbers, tag(" | "), parse_numbers),
            ),
            |(winning, our_numbers)| Input {
                winning,
                our_numbers,
            },
        )(input)
    }

    pub fn parse_input(input: &str) -> Result<Vec<Input>, Error<&str>> {
        input
            .lines()
            .map(|line| parse_line(line).finish().map(|x| x.1))
            .collect()
    }
}

fn get_points(input: &Input) -> u32 {
    input.winning.iter().fold(0, |acc, winner| {
        if input.our_numbers.contains(winner) {
            if acc == 0 {
                1
            } else {
                acc * 2
            }
        } else {
            acc
        }
    })
}

fn solve_part1(input: &[Input]) -> u32 {
    input.iter().map(get_points).sum()
}

fn get_winning_number_count(input: &Input) -> usize {
    input.winning.iter().fold(0, |acc, winner| {
        if input.our_numbers.contains(winner) {
            acc + 1
        } else {
            acc
        }
    })
}

fn solve_part2(input: &[Input]) -> u32 {
    let card_counts = input.iter().map(get_winning_number_count).enumerate().fold(
        vec![1; input.len()],
        |mut counts, (index, winning)| {
            // Winning gives us copies of the next winning cards
            if winning > 0 {
                for won in range(index + 1, index + winning + 1) {
                    if won < counts.len() {
                        // Add a number equal to the copies of this card
                        counts[won] += counts[index]
                    }
                }
            }
            counts
        },
    );
    card_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day4.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 13);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day4.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 30);
        Ok(())
    }
}
