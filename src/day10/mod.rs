use std::collections::{HashMap, VecDeque};

pub fn part1(input: &Vec<String>) -> bool {
    let pairs = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let scores = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let score = input
        .iter()
        .map(|line| {
            line.chars()
                .fold(Ok(VecDeque::new()), |acc, c| {
                    let mut stack = acc?;
                    match c {
                        '(' | '[' | '{' | '<' => {
                            stack.push_back(c);
                            Ok(stack)
                        }
                        _ => {
                            let left = pairs.get(&c).expect("unknown right");
                            if Some(*left) == stack.pop_back() {
                                Ok(stack)
                            } else {
                                Err(scores.get(&c).unwrap())
                            }
                        }
                    }
                })
                .map_or_else(|s| s, |_| &0)
        })
        .sum::<u32>();

    score == 339477
}

pub fn part2(input: &Vec<String>) -> bool {
    let pairs = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let scores = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let mut res = input
        .iter()
        .map(|line| {
            line.chars().fold(Some(VecDeque::new()), |acc, c| {
                let mut stack = acc?;
                match c {
                    '(' | '[' | '{' | '<' => {
                        stack.push_back(c);
                        Some(stack)
                    }
                    _ => {
                        let left = pairs.get(&c).expect("unknown right");
                        if Some(*left) == stack.pop_back() {
                            Some(stack)
                        } else {
                            None
                        }
                    }
                }
            })
        })
        .flatten()
        .filter(|s| s.len() > 0)
        .map(|mut s| {
            let mut score = 0;

            while let Some(left) = s.pop_back() {
                score = score * 5 + scores.get(&left).unwrap()
            }

            score
        })
        .collect::<Vec<u64>>();

    res.sort();
    let score = res[res.len() / 2];
    score == 3049320156
}
