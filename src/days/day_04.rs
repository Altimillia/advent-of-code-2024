use std::collections::HashMap;
use std::fmt::Display;
use crate::domain::point::Point;

pub fn part_one(input: String) -> impl Display {
    let grid = WordGrid::parse(input);

    let result = grid.search_word("XMAS");
    result
}

pub fn part_two(input: String) -> impl Display {
    0
}

struct WordGrid {
    grid: HashMap<Point, char>,
    size: Point
}

impl WordGrid {
    fn parse(input: String) -> Self {
        let mut y_index = (input.lines().count() as i32);
        let mut map:HashMap<Point, char> = HashMap::new();
        let mut total_size:Point = Point::parse(input.lines().nth(0).unwrap().chars().count(), y_index as usize);

        y_index -= 1;
        for (y, line) in input.lines().enumerate() {
            for (x, node) in line.chars().enumerate() {

                map.insert(Point::new(x as i32, y_index as i32), node);
            }
            y_index = y_index - 1;

        }

        WordGrid { grid: map, size: total_size }
    }

    fn search_word(&self, word: &str) -> i32 {
        let first_letter = word.chars().nth(0).unwrap();
        let mut count = 0;
        for y in (0..self.size.y).rev() {
            for x in 0..self.size.x {
                let current_point = Point::new(x,y);
                if first_letter == *self.grid.get(&Point::new(x,y)).unwrap() {

                    let neighbors = Point::new(x,y).get_neighbors();
                    for neighbor in neighbors {
                        count += self.search_for_next_letter_recursive(Point::new(x, y), current_point - neighbor, 1, word);
                    }
                    // count += self.search_for_next_letter_recursive(Point::new(x, y), 1, word);
                    // search the next letters
                }
            }
        }

        count
    }

    fn search_for_next_letter_recursive(&self, current_spot: Point, direction: Point, word_index: usize, word: &str) -> i32 {
        let mut count = 0;
        let next_point = current_spot + direction;
        let letter = word.chars().nth(word_index).unwrap();
        if(!self.grid.contains_key(&next_point)) {
            return 0;
        }

        if letter == *self.grid.get(&next_point).unwrap() {
            if word_index == word.len() - 1 {
                // End of word. LFG
                count += 1;
            }
            else {
                count += self.search_for_next_letter_recursive(next_point, direction, word_index + 1, word);
            }
        }


        count
    }
}
#[cfg(test)]
mod tests {
    use super::{WordGrid};

    #[test]
    fn finding_xmas_in_word_grid() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        let word_grid = WordGrid::parse(input.to_string());
        let result = word_grid.search_word("XMAS");
        assert_eq!(result, 18);
    }
}