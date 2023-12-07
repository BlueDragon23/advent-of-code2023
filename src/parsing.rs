use std::str::FromStr;

use nom::{
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

pub fn parse_number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, |x: &str| x.parse::<T>())(input)
}

/// Parse a list of whitespace separated numbers
pub fn parse_numbers<T: FromStr>(input: &str) -> IResult<&str, Vec<T>> {
    separated_list1(space1, parse_number)(input)
}
