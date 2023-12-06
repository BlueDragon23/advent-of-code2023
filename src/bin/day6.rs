use color_eyre::Result;
use num::{pow, Float};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Input {
    races: Vec<Race>,
}

#[derive(Debug, Clone)]
pub struct Race {
    time: i64,
    distance: i64,
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day6.txt"))?;
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

    use crate::Race;

    use super::Input;
    use advent_of_code2023::parsing::parse_i64;
    use color_eyre::Result;
    use itertools::Itertools;
    use nom::bytes::complete::take_till;
    use nom::character::complete::space1;
    use nom::error::Error;
    use nom::multi::separated_list1;
    use nom::sequence::{pair, preceded};
    use nom::{combinator::map, Finish, IResult};

    fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
        preceded(
            take_till(|c: char| c.is_ascii_digit()),
            separated_list1(space1, parse_i64),
        )(input)
    }

    pub fn parse_input(input: &str) -> Result<Input, Error<&str>> {
        map(pair(parse_line, parse_line), |(times, distances)| Input {
            races: times
                .iter()
                .zip(distances.iter())
                .map(|(&time, &distance)| Race { time, distance })
                .collect_vec(),
        })(input)
        .finish()
        .map(|r| r.1)
    }
}

fn safe_ceil(x: f64) -> i64 {
    if x.ceil() == x {
        (x as i64) + 1
    } else {
        x.ceil() as i64
    }
}

fn safe_floor(x: f64) -> i64 {
    if x.floor() == x {
        (x as i64) - 1
    } else {
        x.floor() as i64
    }
}

fn quadratic_formula(time: i64, record_distance: i64) -> (i64, i64) {
    let float_time = time as f64;
    let float_distance = record_distance as f64;
    let a: f64 = (-float_time + Float::sqrt(pow(float_time, 2) - (4.0 * float_distance))) / (-2.0);
    let b: f64 = (-float_time - Float::sqrt(pow(float_time, 2) - (4.0 * float_distance))) / (-2.0);
    if a < b {
        (safe_ceil(a), safe_floor(b))
    } else {
        (safe_ceil(b), safe_floor(a))
    }
}

fn solve_part1(input: &Input) -> i64 {
    // For each race, iter through time holding button from 0 -> max time, calculate distance travelled.
    // Distance = t * (total_time - t), meaning it has a quadratic shape
    // Distance = t * total_time - t^2
    // Need to solve for distance = record_distance
    // 0 = -t^2 + t*total_time - record_distance
    // Wow, who'd have thought the quadratic formula would come up
    // t = (-total_time +- sqrt(total_time^2 - 4 * record_distance))/(-2)
    // Round the lower value up, upper value down
    // Look out for exact values, since we need to _beat_ the distance
    input
        .races
        .iter()
        .map(|race| quadratic_formula(race.time, race.distance))
        .map(|(lowest, highest)| highest - lowest + 1)
        .product::<i64>()
}

fn solve_part2(input: &Input) -> i64 {
    // Thank goodness for part 1
    let (combined_time, combined_distance) = input.races.iter().fold(
        ("".to_string(), "".to_string()),
        |(combined_time, combined_distance), race| {
            let time_string = race.time.to_string().to_owned();
            let distance_string = race.distance.to_string();
            (
                format!("{combined_time}{time_string}"),
                format!("{combined_distance}{distance_string}"),
            )
        },
    );
    let (time, distance) = (
        combined_time.parse::<i64>().unwrap(),
        combined_distance.parse::<i64>().unwrap(),
    );
    let (lower, upper) = quadratic_formula(time, distance);
    upper - lower + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day6.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 288);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day6.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 71503);
        Ok(())
    }
}
