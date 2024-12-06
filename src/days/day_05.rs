use std::fmt;
use std::fmt::Display;
use std::ops::Index;
use crate::tools::parse_numbers_i32;

pub fn part_one(input: String) -> impl Display {
    let mut split = input.split("\n\n");
    let rules:Vec<Rule> = split.nth(0).unwrap().lines().map(|line| Rule::parse(line)).collect();
    let rule_engine = RuleEngine { rules };
    let manual_updates:Vec<ManualUpdate> = split.nth(0).unwrap().lines().map(|line| ManualUpdate::parse(line)).collect();


    let sum = rule_engine.process_manual_updates(manual_updates).iter().map(|m| m.get_middle_number()).sum::<i32>();
    sum
}

pub fn part_two(input: String) -> impl Display {
    let mut split = input.split("\n\n");
    let rules:Vec<Rule> = split.nth(0).unwrap().lines().map(|line| Rule::parse(line)).collect();
    let rule_engine = RuleEngine { rules };
    let manual_updates:Vec<ManualUpdate> = split.nth(0).unwrap().lines().map(|line| ManualUpdate::parse(line)).collect();


    let passed = rule_engine.get_incorrect_manual_updates(manual_updates);

    let mut updated_list:Vec<ManualUpdate> = Vec::new();

    for update in passed {
        updated_list.push(update.reorder_pages(&rule_engine.rules));
    }

    let total = updated_list.iter().map(|m| m.get_middle_number()).sum::<i32>();

    total
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Rule {
    before_page: i32,
    after_page: i32
}

impl Rule {
    fn parse(input_line: &str) -> Self {
        let values:Vec<i32> = input_line.split("|").map(|v| parse_numbers_i32(v).unwrap().1).collect();

        Rule { before_page: *values.iter().nth(0).unwrap(), after_page: *values.iter().nth(1).unwrap()}
    }


    fn does_rule_apply(&self, manual_update: &ManualUpdate) -> bool {
        if(manual_update.pages.contains(&self.after_page) && manual_update.pages.contains(&self.before_page)){
            return true;
        }

        false
    }

    fn does_rule_pass(&self, manual_update: &ManualUpdate) -> bool {
        if !self.does_rule_apply(manual_update) {
            return true;
        }

        if manual_update.pages.iter().position(|&r| r == self.before_page) < manual_update.pages.iter().position(|&r| r == self.after_page) {
            return true
        }

        false
    }


}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct ManualUpdate {
    pages: Vec<i32>
}

impl ManualUpdate {
    fn parse(input_line: &str) -> Self {
        let values = input_line.split(",").map(|v| parse_numbers_i32(v).unwrap().1).collect();

        ManualUpdate { pages: values}
    }
    fn print(&self) {
        for x in &self.pages {
            print!("{}, ",x);
        }
        println!("");
    }

    fn get_middle_number(&self) -> i32 {
        let mid_index = (self.pages.len() - 1) / 2;
        *self.pages.iter().nth(mid_index).unwrap()
    }

    fn reorder_pages(&self, rules: &Vec<Rule>) -> Self {
        let mut updated_pages = self.clone();

        for rule in rules {
            if(rule.does_rule_pass(&updated_pages)) {
                continue;
            }

            println!("{} {}", rule.before_page, rule.after_page);
            // If the rule does not pass. Then we move the pages to make it work.else
            // Take the before page and move it to the index before the after page. Pushing all other pages down one.
            updated_pages.pages.remove(updated_pages.pages.iter().position(|p| p == &rule.before_page).unwrap());
            updated_pages.pages.insert(updated_pages.pages.iter().position(|p| p == &rule.after_page).unwrap(), rule.before_page);

        }

        for rule in rules {
            if(rule.does_rule_pass(&updated_pages)) {
                continue;
            }

            println!("{} {}", rule.before_page, rule.after_page);
            // If the rule does not pass. Then we move the pages to make it work.else
            // Take the before page and move it to the index before the after page. Pushing all other pages down one.
            updated_pages.pages.remove(updated_pages.pages.iter().position(|p| p == &rule.before_page).unwrap());
            updated_pages.pages.insert(updated_pages.pages.iter().position(|p| p == &rule.after_page).unwrap(), rule.before_page);

        }

        updated_pages
    }
}

struct RuleEngine {
    rules: Vec<Rule>
}

impl RuleEngine {
    fn does_manual_update_pass_rules(&self, manual_update: &ManualUpdate) -> bool {

        for rule in &self.rules {
            if(!rule.does_rule_pass(manual_update)){
                return false;
            }
        }

        true
    }


    fn process_manual_updates(&self, manual_updates: Vec<ManualUpdate>) -> Vec<ManualUpdate> {
        manual_updates.iter().filter(|m| self.does_manual_update_pass_rules(m)).map(|f| f.clone()).collect()
    }

    fn get_incorrect_manual_updates(&self, manual_updates: Vec<ManualUpdate>) -> Vec<ManualUpdate> {
        manual_updates.iter().filter(|m| !self.does_manual_update_pass_rules(m)).map(|f| f.clone()).collect()
    }
}


#[cfg(test)]
mod tests {
    use super::{Rule, RuleEngine};
    use super::{ManualUpdate};

    #[test]
    fn finding_valid_manual_updates() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        let mut split = input.split("\n\n");
        let rules:Vec<Rule> = split.nth(0).unwrap().lines().map(|line| Rule::parse(line)).collect();
        let rule_engine = RuleEngine { rules };
        let manual_updates:Vec<ManualUpdate> = split.nth(0).unwrap().lines().map(|line| ManualUpdate::parse(line)).collect();


        let passed = rule_engine.process_manual_updates(manual_updates);

        assert_eq!(passed.len(), 3);
    }

    #[test]
    fn finding_sum_manual_updates() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        let mut split = input.split("\n\n");
        let rules:Vec<Rule> = split.nth(0).unwrap().lines().map(|line| Rule::parse(line)).collect();
        let rule_engine = RuleEngine { rules };
        let manual_updates:Vec<ManualUpdate> = split.nth(0).unwrap().lines().map(|line| ManualUpdate::parse(line)).collect();


        let sum = rule_engine.process_manual_updates(manual_updates).iter().map(|m| m.get_middle_number()).sum::<i32>();

        assert_eq!(sum, 143);
    }

    #[test]
    fn finding_count_of_incorrect_updates() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
61,13,29
97,13,75,29,47"#;

        let mut split = input.split("\n\n");
        let rules:Vec<Rule> = split.nth(0).unwrap().lines().map(|line| Rule::parse(line)).collect();
        let rule_engine = RuleEngine { rules };
        let manual_updates:Vec<ManualUpdate> = split.nth(0).unwrap().lines().map(|line| ManualUpdate::parse(line)).collect();


        let passed = rule_engine.get_incorrect_manual_updates(manual_updates);

        assert_eq!(passed.len(), 2);
    }

    #[test]
    fn reorder_incorrect_updates() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        let mut split = input.split("\n\n");
        let rules:Vec<Rule> = split.nth(0).unwrap().lines().map(|line| Rule::parse(line)).collect();
        let rule_engine = RuleEngine { rules };
        let manual_updates:Vec<ManualUpdate> = split.nth(0).unwrap().lines().map(|line| ManualUpdate::parse(line)).collect();


        let passed = rule_engine.get_incorrect_manual_updates(manual_updates);

        let mut updated_list:Vec<ManualUpdate> = Vec::new();

        for update in passed {
            updated_list.push(update.reorder_pages(&rule_engine.rules));
        }

        let total = updated_list.iter().map(|m| m.get_middle_number()).sum::<i32>();
        updated_list.iter().for_each(|f| f.print());

        assert_eq!(total, 123);
    }

    #[test]
    fn reorder_list_with_multiple_rules() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

97,13,75,29,47"#;
        let mut split = input.split("\n\n");
        let rules: Vec<Rule> = split.nth(0).unwrap().lines().map(|line| Rule::parse(line)).collect();
        let rule_engine = RuleEngine { rules };
        let manual_updates: Vec<ManualUpdate> = split.nth(0).unwrap().lines().map(|line| ManualUpdate::parse(line)).collect();


        let passed = rule_engine.get_incorrect_manual_updates(manual_updates);

        let mut updated_list: Vec<ManualUpdate> = Vec::new();

        for update in passed {
            updated_list.push(update.reorder_pages(&rule_engine.rules));
        }

        let total = updated_list.iter().map(|m| m.get_middle_number()).sum::<i32>();
        updated_list.iter().for_each(|f| f.print());

        assert_eq!(total, 47);
    }
}