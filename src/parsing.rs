use nom::{character::complete::digit1, combinator::map, IResult};

pub fn parse_u32(input: &str) -> IResult<&str, u32> {
    map(digit1, |x: &str| x.parse::<u32>().unwrap())(input)
}

pub fn parse_u64(input: &str) -> IResult<&str, u64> {
    map(digit1, |x: &str| x.parse::<u64>().unwrap())(input)
}

pub fn parse_i32(input: &str) -> IResult<&str, i32> {
    map(digit1, |x: &str| x.parse::<i32>().unwrap())(input)
}

pub fn parse_i64(input: &str) -> IResult<&str, i64> {
    map(digit1, |x: &str| x.parse::<i64>().unwrap())(input)
}
