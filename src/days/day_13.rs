use crate::tools::parse_numbers_i128;
use itertools::Itertools;
use std::fmt::Display;

pub fn part_one(input: String) -> impl Display {

    let total = parse_all_crane_instructions(input.to_string(), 0).iter().map(|x| calculate_token_cost(x.equation())).sum::<i128>();
    total
}

pub fn part_two(input: String) -> impl Display {
    let total = parse_all_crane_instructions(input.to_string(), 10000000000000).iter().map(|x| calculate_token_cost_without_limit(x.equation())).sum::<i128>();
    total
}

fn calculate_token_cost(presses: Option<(i128, i128)>) -> i128 {
    match presses {
        None => {
            0
        }
        Some((a, b)) => {
            if (a > 100 || b > 100) {
                return 0;
            }
            a * 3 + b * 1
        }
    }
}

fn calculate_token_cost_without_limit(presses: Option<(i128, i128)>) -> i128 {
    match presses {
        None => {
            0
        }
        Some((a, b)) => {
            a * 3 + b * 1
        }
    }
}


fn parse_all_crane_instructions(input: String, prize_add: i128) -> Vec<CraneInstruction> {
    let mut chunks = input.lines().chunks(4);
    chunks.into_iter().map(|mut chunk| {
       parse_crane_instruction(chunk.next().unwrap(), chunk.next().unwrap(), chunk.next().unwrap(), prize_add)
    }).collect()
}
fn parse_crane_instruction(a_button_line: &str, b_button_line: &str, prize_line: &str, prize_add: i128) -> CraneInstruction {

    let a_button = get_button_values(a_button_line);
    let b_button = get_button_values(b_button_line);
    let prize = get_prize_value(prize_line, prize_add);
    CraneInstruction { a_button,b_button, prize }
}

fn get_button_values(button_line: &str) -> (i128, i128) {
    let mut button_line = button_line.replace("Button ", "");
    let mut splits = button_line.split_whitespace();
    let x_value = parse_numbers_i128(&splits.nth(1).unwrap().replace("X+", "")).unwrap().1;
    let y_value = parse_numbers_i128(&splits.nth(0).unwrap().replace("Y+", "")).unwrap().1;

    (x_value, y_value)
}

fn get_prize_value(prize_line: &str, prize_add: i128) -> (i128, i128) {
    let mut button_line = prize_line.replace("Prize: ", "");
    let mut splits = button_line.split_whitespace();
    let x_value = parse_numbers_i128(&splits.nth(0).unwrap().replace("X=", "")).unwrap().1;
    let y_value = parse_numbers_i128(&splits.nth(0).unwrap().replace("Y=", "")).unwrap().1;
    (x_value + prize_add, y_value + prize_add)
}

struct CraneInstruction {
    a_button: (i128, i128),
    b_button: (i128, i128),
    prize: (i128, i128)
}

// ax + bx = px
// ay + by = py

// a94 + b22 = 8400
// a34 + b67 = 5400


//Button A: X+94, Y+34
//Button B: X+22, Y+67
//Prize: X=8400, Y=5400



impl CraneInstruction {

    fn equation(&self) -> Option<(i128, i128)> {
        let delta = self.a_button.0 * self.b_button.1 - self.b_button.0 * self.a_button.1;

        if delta == 0 {
            return None;
        }

        let a_num = self.prize.0 * self.b_button.1 - self.prize.1 * self.b_button.0;
        let b_num = self.a_button.0 * self.prize.1 - self.a_button.1 * self.prize.0;

        if a_num % delta != 0 || b_num % delta != 0 {
            return None;
        }

        let a = a_num / delta;
        let b = b_num / delta;

        Some((a, b))
    }
}
#[cfg(test)]
mod tests {
    use crate::days::day_13::{calculate_token_cost, parse_all_crane_instructions};

    #[test]
    fn can_calulate_a_b_presses() {
        let input = r#"Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450"#;

        let cranes = parse_all_crane_instructions(input.to_string(),0);

        let result = cranes.iter().nth(0).unwrap().equation();

        assert_eq!(result.unwrap().0, 38);
        assert_eq!(result.unwrap().1, 86);
    }

    #[test]
    fn can_calculate_token_cost() {
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400"#;

        let cranes = parse_all_crane_instructions(input.to_string(),0);

        let result = cranes.iter().nth(0).unwrap().equation();
        let cost = calculate_token_cost(result);
        assert_eq!(cost, 280);
    }

    #[test]
    fn can_get_total_token_cost() {
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

        let total = parse_all_crane_instructions(input.to_string(),0).iter().map(|x| calculate_token_cost(x.equation())).sum::<i128>();
        assert_eq!(total, 480);
    }
}
