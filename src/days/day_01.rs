use std::fmt::Display;
use crate::tools::{parse_numbers_i32, usize_to_i32};
pub fn part_one(input: String) -> impl Display {
    let instructions:Vec<(&str, &str)> = input
        .lines()
        .into_iter()
        .map(|f| split_line(f))
        .collect();

    let mut left_list:Vec<i32> = Vec::new();
    let mut right_list:Vec<i32> = Vec::new();

    instructions.iter().for_each(|(l, r)| {
        left_list.push(parse_numbers_i32(l).unwrap().1);
        right_list.push(parse_numbers_i32(r).unwrap().1);
    });

    left_list.sort();
    right_list.sort();
    let mut total = 0;
    for x in 0..left_list.len() {
        let difference = left_list[x] - right_list[x];
        total += difference.abs();
    }
    total
}

pub fn part_two(input: String) -> impl Display {
    let instructions:Vec<(&str, &str)> = input
        .lines()
        .into_iter()
        .map(|f| split_line(f))
        .collect();

    let mut left_list:Vec<i32> = Vec::new();
    let mut right_list:Vec<i32> = Vec::new();

    instructions.iter().for_each(|(l, r)| {
        left_list.push(parse_numbers_i32(l).unwrap().1);
        right_list.push(parse_numbers_i32(r).unwrap().1);
    });

    left_list.sort();
    right_list.sort();

    let mut total:i32 = 0;

    left_list.iter().for_each(|v| {
        let matches = right_list.clone().into_iter().filter(|x| x == v).count();
        total += (v * usize_to_i32(matches).unwrap());
    });

    total
}

fn split_line(input: &str) -> (&str, &str) {
    let mut entries = input.split_whitespace();

    let left = entries.next();
    let right = entries.next();

    (left.unwrap(), right.unwrap())
}

struct LocationIdRange {
    value: i32
}