use std::fmt::{Debug, Display};
use std::ops::Index;
use nom::bytes::complete::{tag, take_until};
use nom::IResult;
use nom::sequence::{delimited, separated_pair};
use nom::character::complete::char;
use nom::combinator::{opt, peek};
use crate::tools::parse_numbers_i32;

pub fn part_one(input: String) -> impl Display {
    let results = parse_out_operations(&input).unwrap().1;

    let total = results.iter().map(|x| x.get_result()).sum::<i32>();
    total
}

pub fn part_two(input: String) -> impl Display {
    let results = parse_out_operations_advanced(&input).unwrap().1;

    let total = results.iter().map(|x| x.get_result()).sum::<i32>();
    total
}


fn parse_out_operations(input: &str) -> IResult<&str, Vec<MultiplyOperation>> {
    let mut operations: Vec<MultiplyOperation> = Vec::new();
    let mut remaining_input = input;
    let mut input = input;

    loop {


        let (input, _) = take_until("mul(")(remaining_input)?;
        let (input, _) = tag("mul")(input)?;
        let (input, operation) = opt(parse_out_next)(input)?;
        match operation {
            None => {}
            Some(operation_value) => {
                    operations.push(operation_value);
            }
        }
        remaining_input = input;
        let check = check_pattern(remaining_input);

        let peek_result = check;
        if(!peek_result.is_ok()){
            break;
        }
    }

    Ok((input, operations))
}


fn parse_out_operations_advanced(input: &str) -> IResult<&str, Vec<MultiplyOperation>> {
    let mut operations: Vec<MultiplyOperation> = Vec::new();
    let mut remaining_input = input;
    let mut input = input;
    let mut enabled = true;
    loop {

        if(enabled){
            let dont_match = remaining_input.find("don't()");
            let mut dont_index = 0;
            match dont_match {
                None => {}
                Some(v) => {
                    dont_index = v;
                }
            }


            let mul_match = remaining_input.find("mul(");
            match mul_match {
                None => {}
                Some(index) => {
                    if(dont_index > index) {
                        // its a mul first. We dont do anything
                    }
                    else if(dont_index != 0 && dont_index < index){
                        enabled = false;
                        let (input, _) = take_until("don't()")(remaining_input)?;
                        let (input, _) = tag("don't()")(input)?;
                        remaining_input = input;
                    }
                }
            }
            // Check if mul( or don't() occur first.
            // If don't(), then consume up to it and disable
            // If mul( then don't do anything
        }
        else {
            let do_match = remaining_input.find("do()");
            let mut do_index = 0;
            match do_match {
                None => {}
                Some(v) => {
                    enabled = true;
                    let (input, _) = take_until("do()")(remaining_input)?;
                    let (input, _) = tag("do()")(input)?;
                    remaining_input = input;
                }
            }
        }

        let (input, _) = take_until("mul(")(remaining_input)?;
        let (input, _) = tag("mul")(input)?;
        let (input, operation) = opt(parse_out_next)(input)?;
        match operation {
            None => {}
            Some(operation_value) => {
                if(enabled) {
                    operations.push(operation_value);
                }
            }
        }
        remaining_input = input;
        let check = check_pattern(remaining_input);

        let peek_result = check;
        if(!peek_result.is_ok()){
            break;
        }
    }

    Ok((input, operations))
}

fn check_pattern(input: &str) -> IResult<&str, &str> {
    peek(take_until("mul"))(input)
}


fn parse_out_next(input: &str) -> IResult<&str, MultiplyOperation> {
    let (input_line, (left,right)) = delimited(
        char('('),
        separated_pair(parse_numbers_i32, char(','), parse_numbers_i32),
        char(')')
    )(input)?;

    Ok((input_line, MultiplyOperation { left: left, right: right }))
}

#[derive(Debug, Clone, Copy)]
struct MultiplyOperation{
    left: i32,
    right: i32
}

impl MultiplyOperation {
    fn get_result(&self) -> i32 {
        &self.left * &self.right
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day_03;

    #[test]
    fn multiply_operations_can_be_parsed() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        let results = day_03::parse_out_operations(input).unwrap().1;

        assert_eq!(results.len(), 4);
    }

    #[test]
    fn multiply_operations_can_be_summed() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        let results = day_03::parse_out_operations(input).unwrap().1;

        let total = results.iter().map(|x| x.get_result()).sum::<i32>();
        assert_eq!(total, 161);
    }

    #[test]
    fn multiply_operations_can_be_summed_with_advanced() {
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        let results = day_03::parse_out_operations_advanced(input).unwrap().1;

        let total = results.iter().map(|x| x.get_result()).sum::<i32>();
        assert_eq!(total, 48);
    }
}