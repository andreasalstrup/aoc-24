use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::File;
use std::io::{prelude::*, Result};

#[derive(Clone, Debug)]
struct Rule {
    x: u32,
    y: u32,
}

impl Rule {
    fn new(rule: Vec<u32>) -> Self {
        let rule: Vec<Rule> = rule
            .iter()
            .step_by(2)
            .enumerate()
            .map(|(i, num)| Rule {
                x: *num,
                y: rule[i + 1],
            })
            .collect();

        rule[0].clone()
    }
}

#[derive(Debug)]
struct Update {
    data: Vec<u32>,
    right_order: Option<bool>,
}

impl Update {
    fn new(data: Vec<u32>) -> Self {
        Update {
            data,
            right_order: None,
        }
    }
}

#[derive(Debug)]
struct SafetyManual {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

impl SafetyManual {
    fn new(buffer: &str) -> Self {
        let mut lines = buffer.lines();

        let rules: Vec<Rule> = lines
            .by_ref()
            .take_while(|&line| !line.is_empty())
            .map(|line| Rule::new(line.split('|').map(|r| r.parse::<u32>().unwrap()).collect()))
            .collect();

        let updates: Vec<Update> = lines
            .map(|line| Update::new(line.split(',').map(|r| r.parse::<u32>().unwrap()).collect()))
            .collect();

        SafetyManual { rules, updates }
    }

    fn safe_check(&mut self) {
        for update in self.updates.iter_mut() {
            for (i, num) in update.data.iter().enumerate() {
                let should_come_before: HashSet<u32> = self
                    .rules
                    .iter()
                    .filter(|&r| r.x == *num)
                    .map(|r| r.y)
                    .collect();

                let mut y_set = HashSet::new();
                let _ = &update.data[0..i].iter().for_each(|n| {
                    y_set.insert(*n);
                });

                let x: HashSet<_> = should_come_before.intersection(&y_set).collect();
                update.right_order = Some(x.is_empty());
                if !x.is_empty() {
                    break;
                }
            }
        }
    }

    fn part_one(&mut self) -> u32 {
        self.safe_check();
        self.updates
            .iter()
            .filter(|u| u.right_order.is_some_and(|x| x))
            .fold(0, |acc, u| acc + u.data[u.data.len() / 2])
    }
}

fn main() -> Result<()> {
    let mut file = File::open("./day-5/input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let mut safety_manual = SafetyManual::new(&buffer);
    println!("Part one: {}", safety_manual.part_one());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let test = "47|53
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
97,13,75,29,47";

        let mut safety_manual = SafetyManual::new(test);

        safety_manual.safe_check();

        assert_eq!(safety_manual.updates[0].right_order, Some(true));
        assert_eq!(safety_manual.updates[1].right_order, Some(true));
        assert_eq!(safety_manual.updates[2].right_order, Some(true));
        assert_eq!(safety_manual.updates[3].right_order, Some(false));
        assert_eq!(safety_manual.updates[4].right_order, Some(false));
        assert_eq!(safety_manual.updates[5].right_order, Some(false));

        assert_eq!(safety_manual.part_one(), 143);
    }
}