use color_eyre::Result;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Input {}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../input/example.txt"))?;
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
    use nom::error::Error;
    use nom::{bytes::complete::tag, combinator::map, Finish, IResult};

    fn parse_line(input: &str) -> IResult<&str, Input> {
        map(tag(" -> "), |_| Input {})(input)
    }

    pub fn parse_input(input: &str) -> Result<Vec<Input>, Error<&str>> {
        input
            .lines()
            .map(|line| parse_line(line).finish().map(|x| x.1))
            .collect()
    }
}

fn solve_part1(input: &[Input]) -> u32 {
    1
}

fn solve_part2(input: &[Input]) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../input/example.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 1);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../input/example.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 1);
        Ok(())
    }
}
