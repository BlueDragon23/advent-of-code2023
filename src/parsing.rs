use std::str::FromStr;

use nom::{
    character::complete::{digit1, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    IResult,
};

pub fn parse_number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, |x: &str| x.parse::<T>())(input)
}

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

/// Parse a list of whitespace separated numbers
pub fn parse_numbers<T: FromStr>(input: &str) -> IResult<&str, Vec<T>> {
    separated_list1(space1, parse_number)(input)
}
