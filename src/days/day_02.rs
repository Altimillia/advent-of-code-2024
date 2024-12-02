use std::fmt::Display;
use crate::tools::{parse_numbers_i32, parse_numbers_i64};

pub fn part_one(input: String) -> impl Display {
    let valid = input.lines().map(|line| Report::parse(line))
        .filter(|report| report.is_safe(&report.levels)).into_iter().count();


    valid
}

pub fn part_two(input: String) -> impl Display {
    let valid = input.lines().map(|line| Report::parse(line))
        .filter(|report| report.is_safe_with_problem_dampener()).into_iter().count();


    valid
}

struct Report{
    levels: Vec<i32>
}

impl Report {
    fn parse(input_line: &str) -> Self {
        let numbers:Vec<i32> = input_line.split_whitespace().map(|number| parse_numbers_i32(number).unwrap().1).collect();

        Report { levels: numbers }
    }

    fn is_safe(&self, levels: &Vec<i32>) -> bool {
        let mut windows = levels.windows(2);

        let mut differences:Vec<i32> = windows.into_iter()
            .map(|x| x[1] - x[0]).collect();


        let all_changes = differences.iter().all(|f| *f > 0) || differences.iter().all(|f| *f < 0);
        let no_large_jumps = differences.iter().all(|f| f.abs() < 4);

        all_changes && no_large_jumps
    }

    fn is_safe_with_problem_dampener(&self) -> bool {
        let initial = self.is_safe(&self.levels);

        if initial {
            return initial;
        }

        for index in 0..self.levels.len() {
            let mut vec = self.levels.clone();
            vec.remove(index);
            let damp_check = self.is_safe(&vec);
            if(damp_check){
                return damp_check;
            }
        }

        false
    }

}
#[cfg(test)]
mod tests {
    use super::{Report};


    #[test]
    fn reports_can_be_found_safe() {
        let input = r#"7 6 4 2 1"#;
        let report = Report::parse(input);

        assert_eq!(report.is_safe(&report.levels), true);
    }

    #[test]
    fn reports_can_be_found_un_safe_due_to_large_increase() {
        let input = r#"1 2 7 8 9"#;
        let report = Report::parse(input);

        assert_eq!(report.is_safe(&report.levels), false);
    }

    #[test]
    fn reports_can_be_found_un_safe_due_to_change_in_direction() {
        let input = r#"1 3 2 4 5"#;
        let report = Report::parse(input);

        assert_eq!(report.is_safe(&report.levels), false);
    }

    #[test]
    fn report_can_be_found_safe_by_removing_level(){
        let input = r#"1 3 2 4 5"#;
        let report = Report::parse(input);

        assert_eq!(report.is_safe_with_problem_dampener(), true);
    }

    #[test]
    fn reports_can_still_be_unsafe_with_dampener(){
        let input = r#"9 7 6 2 1"#;
        let report = Report::parse(input);

        assert_eq!(report.is_safe_with_problem_dampener(), false);
    }
}
