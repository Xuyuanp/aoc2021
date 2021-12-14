use std::collections::HashMap;

fn parse_input(input: &Vec<String>) -> (String, HashMap<String, char>) {
    let mut rules = HashMap::new();

    let templ = input.iter().next().unwrap().to_string();

    input[2..].iter().for_each(|line| {
        let seps: Vec<&str> = line.split(" -> ").collect();
        rules.insert(seps[0].to_string(), seps[1].chars().next().unwrap());
    });

    (templ, rules)
}

fn run_steps(templ: String, rules: HashMap<String, char>, steps: usize) -> u64 {
    let mut letters = HashMap::new();
    let mut pairs = HashMap::new();

    templ
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .for_each(|chars| {
            *pairs.entry(chars.iter().collect::<String>()).or_insert(0) += 1;
            *letters.entry(chars[0]).or_insert(0) += 1;
        });

    *letters.entry(templ.chars().last().unwrap()).or_insert(0) += 1;

    for _ in 0..steps {
        let mut new_pairs: HashMap<String, u64> = HashMap::new();
        for (pair, cnt) in pairs.iter() {
            let new_char = *rules.get(pair).unwrap();
            *letters.entry(new_char).or_insert(0) += cnt;
            *new_pairs
                .entry([pair.chars().next().unwrap(), new_char].iter().collect())
                .or_insert(0) += cnt;
            *new_pairs
                .entry([new_char, pair.chars().last().unwrap()].iter().collect())
                .or_insert(0) += cnt;
        }
        pairs = new_pairs;
    }

    letters.values().max().unwrap() - letters.values().min().unwrap()
}

pub fn part1(input: &Vec<String>) -> bool {
    let (templ, rules) = parse_input(input);
    let res = run_steps(templ, rules, 10);

    res == 2112
}

pub fn part2(input: &Vec<String>) -> bool {
    let (templ, rules) = parse_input(input);

    let res = run_steps(templ, rules, 40);

    res == 3243771149914
}
