use nom::{character::complete::digit1, combinator::map, IResult};

pub fn parse_u32(input: &str) -> IResult<&str, u32> {
    map(digit1, |x: &str| x.parse::<u32>().unwrap())(input)
}

pub fn parse_i32(input: &str) -> IResult<&str, i32> {
    map(digit1, |x: &str| x.parse::<i32>().unwrap())(input)
}
