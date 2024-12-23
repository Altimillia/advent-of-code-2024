﻿use std::str::FromStr;
use nom::{IResult, combinator::{map_res, recognize, opt}, sequence::preceded, character::complete::digit1, bytes::complete::tag};
#[allow(dead_code)]
pub fn is_digit(c: &char) -> bool {
    c.is_digit(10)
}

pub fn parse_numbers_i32(input: &str) -> IResult<&str, i32> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        i32::from_str(s) })(input)?;

    Ok((i, number))
}

pub fn parse_numbers_u64(input: &str) -> IResult<&str, u64> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        u64::from_str(s)
    })(input)?;

    Ok((i, number))
}

pub fn parse_numbers_i64(input: &str) -> IResult<&str, i64> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        i64::from_str(s)
    })(input)?;

    Ok((i, number))
}

pub fn parse_numbers_i128(input: &str) -> IResult<&str, i128> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        i128::from_str(s)
    })(input)?;

    Ok((i, number))
}
pub fn parse_numbers_u128(input: &str) -> IResult<&str, u128> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        u128::from_str(s)
    })(input)?;

    Ok((i, number))
}

pub fn usize_to_i32(num: usize) -> Result<i32, String> {
    // Check if the `usize` value can fit within the range of `i32`
    if num > i32::MAX as usize {
        return Err(format!("Invalid input. The value {} is too large to fit within the range of i32.", num));
    }

    // Convert the `usize` value to `i32`
    let converted_num = num as i32;

    Ok(converted_num)
}

pub fn usize_to_u32(num: usize) -> Result<u32, String> {
    // Check if the `usize` value can fit within the range of `u32`
    if num > u32::MAX as usize {
        return Err(format!("Invalid input. The value {} is too large to fit within the range of u32.", num));
    }

    // Convert the `usize` value to `i32`
    let converted_num = num as u32;

    Ok(converted_num)
}

pub fn usize_to_u64(num: usize) -> Result<u64, String> {
    // Check if the `usize` value can fit within the range of `u32`
    if num > u32::MAX as usize {
        return Err(format!("Invalid input. The value {} is too large to fit within the range of u32.", num));
    }

    // Convert the `usize` value to `i32`
    let converted_num = num as u64;

    Ok(converted_num)
}

pub fn usize_to_i64(num: usize) -> Result<i64, String> {
    // Check if the `usize` value can fit within the range of `u32`
    if num > i64::MAX as usize {
        return Err(format!("Invalid input. The value {} is too large to fit within the range of u32.", num));
    }

    // Convert the `usize` value to `i32`
    let converted_num = num as i64;

    Ok(converted_num)
}