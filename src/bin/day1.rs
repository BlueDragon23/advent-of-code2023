use color_eyre::Result;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Input {
    first: u32,
    last: u32,
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day1.txt"));
    let time = Instant::now();
    println!(
        "Part 1: {} in {}ms",
        solve_part1(&input),
        time.elapsed().as_millis()
    );
    let input_2 = parsing::parse_input_2(include_str!("../../input/day1.txt"));
    let time = Instant::now();
    println!(
        "Part 2: {} in {}ms",
        solve_part2(&input_2),
        time.elapsed().as_millis()
    );
    Ok(())
}

mod parsing {

    use super::Input;

    fn parse_line(input: &str) -> Input {
        let first = input
            .chars()
            .find(|x| x.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap();
        let last = input
            .chars()
            .rev()
            .find(|x| x.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap();
        Input { first, last }
    }

    pub fn parse_input(input: &str) -> Vec<Input> {
        input.lines().map(parse_line).collect()
    }

    fn match_word(word: &str) -> u32 {
        // println!("{}", word);
        match word {
            "1" | "one" => 1,
            "2" | "two" => 2,
            "3" | "three" => 3,
            "4" | "four" => 4,
            "5" | "five" => 5,
            "6" | "six" => 6,
            "7" | "seven" => 7,
            "8" | "eight" => 8,
            "9" | "nine" => 9,
            _ => panic!("Illegal"),
        }
    }

    fn parse_line_2(input: &str) -> Input {
        // println!("Input line is {}", input);
        let numbers: Vec<&str> = vec![
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine",
        ];

        // print!("First is: ");
        let first = match_word(
            numbers
                .iter()
                .filter_map(|&num| input.find(num).map(|pos| (num, pos)))
                .min_by_key(|(_, pos)| *pos)
                .unwrap()
                .0,
        );
        // print!("Last is: ");
        let last = match_word(
            numbers
                .iter()
                .filter_map(|&num| input.rfind(num).map(|pos| (num, pos)))
                .max_by_key(|(_, pos)| *pos)
                .unwrap()
                .0,
        );
        Input { first, last }
    }

    pub fn parse_input_2(input: &str) -> Vec<Input> {
        input.lines().map(parse_line_2).collect()
    }
}

fn solve_part1(input: &[Input]) -> u32 {
    input
        .iter()
        .map(|x| format!("{}{}", x.first, x.last).parse::<u32>().unwrap())
        .sum()
}

fn solve_part2(input: &[Input]) -> u32 {
    // println!("{:?}", input);
    input
        .iter()
        .map(|x| format!("{}{}", x.first, x.last).parse::<u32>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day1.test.txt"));
        let result = solve_part1(&input);
        assert_eq!(result, 142);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input_2(include_str!("../../input/day1.test2.txt"));
        let result = solve_part2(&input);
        assert_eq!(result, 281);
        Ok(())
    }
}
