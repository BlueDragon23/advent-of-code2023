pub mod coordinate;
pub mod range;
// Force template to compile
mod template;
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::io::{BufRead, Lines};

use itertools::Itertools;
use reformation::Reformation;

// Example union input
#[derive(Reformation, Eq, PartialEq, Debug)]
#[allow(dead_code)]
enum Ant {
    #[reformation(r"Queen\({}\)")]
    Queen(String),
    #[reformation(r"Worker\({}\)")]
    Worker(i32),
    #[reformation(r"Warrior")]
    Warrior,
}

// Example struct input
#[derive(Reformation, Debug)]
#[reformation(r"{year}-{month}-{day} {hour}:{minute}")]
#[allow(dead_code)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

pub fn group_file_by_empty_lines(reader: BufReader<File>) -> Vec<Vec<String>> {
    reader
        .lines()
        .map(|line| line.unwrap())
        .fold(vec![vec![]], |mut result, line| {
            if line.trim().is_empty() {
                result.push(Vec::new());
                result
            } else {
                result.last_mut().unwrap().push(line);
                result
            }
        })
}

// Create a method for parsing a line of ints
pub fn parse_line_to_num(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect_vec()
}

// Create a method for parsing lines of a file to ints
pub fn parse_lines_to_nums(lines: Lines<BufReader<File>>) -> Vec<i32> {
    lines
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect_vec()
}

// #[allow(dead_code)]
// fn parse_dates(reader: BufReader<File>) -> Vec<Date> {
//     parse_lines_to_struct::<Date>(reader)
// }

// // Create a method for parsing lines of a file to a particular struct using reformation
// #[allow(dead_code)]
// pub fn parse_lines_to_struct<'a, T: Reformation<'a>>(reader: BufReader<File>) -> Vec<T> {
//     reader
//         .lines()
//         .map(|line| T::parse(&line.unwrap()).unwrap())
//         .collect_vec()
// }

pub fn print_matrix<T: Display>(matrix: &[Vec<T>]) {
    for line in matrix {
        println!("{}", line.iter().join(""));
    }
    println!();
}
