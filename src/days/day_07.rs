use std::fmt::Display;
use std::ptr::eq;
use nom::bytes::streaming::take_until;
use nom::combinator::value;
use crate::tools::{parse_numbers_i128, parse_numbers_i64};

pub fn part_one(input: String) -> impl Display {
    let equations:Vec<Equation> = input.lines().map(|line| Equation::parse(line)).filter(|x| attempt_solve(x.clone())).collect();

    equations.iter().map(|x| x.result).sum::<i128>()
}

pub fn part_two(input: String) -> impl Display {
    let equations:Vec<Equation> = input.lines().map(|line| Equation::parse(line)).filter(|x| attempt_solve_part_2(x.clone())).collect();

    equations.iter().map(|x| x.result).sum::<i128>()
}

fn attempt_solve_part_2(equation: Equation) -> bool {

    let combos = generate_combinations_part_2(equation.values.len());

    attempt_to_solve_dynamically(equation, combos.clone())
}

fn attempt_solve(equation: Equation) -> bool {
    let combos = generate_combinations(equation.values.len());

    attempt_to_solve_dynamically(equation, combos.clone())
}

fn attempt_to_solve_dynamically(equation: Equation, combos: Vec<Vec<char>>) -> bool {
    for combo in combos {
        let mut result = *equation.values.iter().nth(0).unwrap();
        for x in 1..equation.values.len() {
            let next = *equation.values.iter().nth(x).unwrap();

            let operator = *combo.iter().nth(x - 1).unwrap();
            match operator
            {
                '*' => {
                    result = result * next;
                },
                '+' => {
                    result = result + next;
                },
                '|' => {
                    result = parse_numbers_i128(&(result.to_string() + &next.to_string())).unwrap().1;
                }
                _ => {}
            }
        }

        if(result == equation.result){
            return true;
        }
        //println!("{}", result);
    }
    false
}

fn generate_combinations(length: usize) -> Vec<Vec<char>> {
    let mut combinations = Vec::new();
    let total_combinations = 1 << length; // 2^length

    for i in 0..total_combinations {
        let mut combination = Vec::new();

        for j in 0..length {
            if (i & (1 << j)) != 0 {
                combination.push('+');
            } else {
                combination.push('*');
            }
        }

        combinations.push(combination);
    }

    combinations
}


fn generate_combinations_part_2(length: usize) -> Vec<Vec<char>> {
    let mut combinations = Vec::new();
    let total_combinations = 3_usize.pow(length as u32);

    for i in 0..total_combinations {
        let mut combination = Vec::new();
        let mut current = i;

        for _ in 0..length {
            match current % 3 {
                0 => combination.push('*'),
                1 => combination.push('+'),
                2 => combination.push('|'),
                _ => unreachable!(),
            }
            current /= 3;
        }

        combinations.push(combination);
    }

    combinations
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Equation
{
    result: i128,
    values: Vec<i128>
}

impl Equation
{
    fn parse(input_line: &str) -> Self {
        let mut result_split = input_line.split(":");
        let result = parse_numbers_i128(result_split.next().unwrap()).unwrap().1;
        let values:Vec<i128> = result_split.next().unwrap().split_whitespace().map(|x| parse_numbers_i128(x).unwrap().1).collect();

        Equation { result, values }
    }
}

#[cfg(test)]
mod tests {
    use std::ptr::eq;
    use crate::days::day_07::{attempt_solve, attempt_solve_part_2, Equation};

    #[test]
    fn can_parse_equation() {
        let input = r#"190: 10 19"#;
        let equation = Equation::parse(input);

        assert_eq!(equation.result, 190);
        assert_eq!(equation.values.len(), 2);
        assert_eq!(*equation.values.iter().nth(0).unwrap(), 10);
        assert_eq!(*equation.values.iter().nth(1).unwrap(), 19);
    }

    #[test]
    fn can_solve_equation() {
        let input = r#"190: 10 19"#;
        let equation = Equation::parse(input);

        let result = attempt_solve(equation);

        assert_eq!(result, true);
    }

    #[test]
    fn can_find_multiple_solutions() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

        let equations:Vec<Equation> = input.lines().map(|line| Equation::parse(line)).filter(|x| attempt_solve(x.clone())).collect();

        assert_eq!(equations.iter().map(|x| x.result).sum::<i128>(), 3749);
    }

    #[test]
    fn part_2_can_find_all_solutions() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;


        let equations:Vec<Equation> = input.lines().map(|line| Equation::parse(line)).filter(|x| attempt_solve_part_2(x.clone())).collect();

        assert_eq!(equations.iter().map(|x| x.result).sum::<i128>(), 11387);
    }
}