use std::fmt::Display;
use crate::tools::parse_numbers_i128;

pub fn part_one(input: String) -> impl Display {
    blink_amount(25, StoneArrangement::parse(input))
}

pub fn part_two(input: String) -> impl Display {
    //blink_amount(25, StoneArrangement::parse(input))
    0
}

fn blink_amount(blink_amount: i32, mut initial_arrangement: StoneArrangement) -> usize {

    for x in 0..blink_amount {
        initial_arrangement = initial_arrangement.blink();
        println!("{} - {}", x, initial_arrangement.stones.len());
    }
    initial_arrangement.stones.len()
}

struct StoneArrangement {
    stones: Vec<i128>
}

impl StoneArrangement {
    fn parse(input: String) -> Self {
        StoneArrangement { stones: input.split_whitespace().map(|s| parse_numbers_i128(s).unwrap().1).collect() }
    }

    fn blink(&self) -> StoneArrangement {
        let mut next_stones:Vec<i128> = Vec::new();

        for stone in &self.stones {
            // Do Rules here
            if(*stone == 0){
                next_stones.push(1);
            }
            else if (stone.to_string().len() % 2 == 0) {
                let stone_string = stone.to_string();
                let splits = stone_string.split_at((stone_string.len() / 2));
                next_stones.push(parse_numbers_i128(splits.0).unwrap().1);
                next_stones.push(parse_numbers_i128(splits.1).unwrap().1);
            }
            else {
                next_stones.push(stone * 2024);
            }

        }

        StoneArrangement { stones: next_stones }
    }

    fn to_arrange_display(&self) -> String {
        let mut owned_string: String = "".to_owned();

        for stone in &self.stones {
            owned_string.push_str(&stone.to_string());
            owned_string.push_str(" ");
        }

        owned_string
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day_11::{blink_amount, StoneArrangement};

    #[test]
    fn stone_arrangement_can_be_blinked() {
        let input = r#"125 17"#;
        let arrangement = StoneArrangement::parse(input.to_string());

        let next = arrangement.blink();

        assert_eq!("253000 1 7", next.to_arrange_display().trim())
    }

    #[test]
    fn stone_arrangement_can_be_blinked_multiple_times() {
        let input = r#"125 17"#;
        let arrangement = StoneArrangement::parse(input.to_string());

        let next = arrangement.blink().blink();

        assert_eq!("253 0 2024 14168", next.to_arrange_display().trim())
    }

    #[test]
    fn twenty_five_blinks() {
        let input = r#"125 17"#;
        let mut arrangement = StoneArrangement::parse(input.to_string());


        assert_eq!(blink_amount(25, arrangement), 55312);
    }

    #[test]
    fn oh_god_seventy_five_blinks() {
        let input = r#"125 17"#;
        let mut arrangement = StoneArrangement::parse(input.to_string());


        assert_eq!(blink_amount(75, arrangement), 55312);
    }
}

