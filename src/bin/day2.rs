use color_eyre::Result;
use std::{cmp::max, time::Instant};

#[derive(Debug, Clone)]
pub struct Input {
    id: u32,
    games: Vec<Balls>,
}

#[derive(Debug, Clone)]
pub struct Balls {
    red: u32,
    green: u32,
    blue: u32,
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day2.txt"))?;
    let total_balls = Balls {
        red: 12,
        green: 13,
        blue: 14,
    };
    let time = Instant::now();
    println!(
        "Part 1: {} in {}ms",
        solve_part1(&input, &total_balls),
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

    use crate::Balls;

    use super::Input;
    use color_eyre::Result;
    use nom::branch::alt;

    use nom::character::complete::digit1;
    use nom::error::Error;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair};
    use nom::{bytes::complete::tag, combinator::map, Finish, IResult};

    #[derive(Debug, Clone)]
    enum Ball {
        Red(u32),
        Green(u32),
        Blue(u32),
    }

    fn parse_number(input: &str) -> IResult<&str, u32> {
        map(digit1, |x: &str| x.parse::<u32>().unwrap())(input)
    }

    fn parse_cube(input: &str) -> IResult<&str, Ball> {
        map(
            separated_pair(
                parse_number,
                tag(" "),
                alt((tag("red"), tag("green"), tag("blue"))),
            ),
            |(number, colour)| match colour {
                "red" => Ball::Red(number),
                "green" => Ball::Green(number),
                "blue" => Ball::Blue(number),
                _ => panic!("Illegal colour {}", colour),
            },
        )(input)
    }

    fn parse_game(input: &str) -> IResult<&str, Balls> {
        map(separated_list1(tag(", "), parse_cube), |balls| {
            if balls.len() > 3 {
                panic!("Invalid balls: {:?}", balls);
            }
            Balls {
                red: balls
                    .iter()
                    .find_map(|b| match b {
                        Ball::Red(x) => Some(*x),
                        _ => None,
                    })
                    .unwrap_or(0),
                green: balls
                    .iter()
                    .find_map(|b| match b {
                        Ball::Green(x) => Some(*x),
                        _ => None,
                    })
                    .unwrap_or(0),
                blue: balls
                    .iter()
                    .find_map(|b| match b {
                        Ball::Blue(x) => Some(*x),
                        _ => None,
                    })
                    .unwrap_or(0),
            }
        })(input)
    }

    fn parse_line(input: &str) -> IResult<&str, Input> {
        map(
            separated_pair(
                preceded(tag("Game "), parse_number),
                tag(": "),
                separated_list1(tag("; "), parse_game),
            ),
            |(id, games)| Input { id, games },
        )(input)
    }

    pub fn parse_input(input: &str) -> Result<Vec<Input>, Error<&str>> {
        input
            .lines()
            .map(|line| parse_line(line).finish().map(|x| x.1))
            .collect()
    }
}

fn is_legal_game(game: &Balls, total_balls: &Balls) -> bool {
    game.blue <= total_balls.blue && game.red <= total_balls.red && game.green <= total_balls.green
}

fn solve_part1(input: &[Input], total_balls: &Balls) -> u32 {
    input
        .iter()
        .filter_map(|input_line| {
            if input_line
                .games
                .iter()
                .all(|game| is_legal_game(game, total_balls))
            {
                Some(input_line.id)
            } else {
                None
            }
        })
        .sum()
}

fn solve_part2(input: &[Input]) -> u32 {
    input
        .iter()
        .map(|input_line| {
            // Find the max by colour in all games
            let (r, g, b) = input_line.games.iter().fold((0, 0, 0), |(r, g, b), game| {
                (max(r, game.red), max(g, game.green), max(b, game.blue))
            });
            r * g * b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day2.test.txt"))?;
        let total_balls = Balls {
            red: 12,
            green: 13,
            blue: 14,
        };
        let result = solve_part1(&input, &total_balls);
        assert_eq!(result, 8);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day2.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 2286);
        Ok(())
    }
}
