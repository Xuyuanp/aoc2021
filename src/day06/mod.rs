use std::collections::BTreeMap;

fn lanternfish(init: &Vec<String>, days: u32) -> u64 {
    let mut timers: BTreeMap<u32, u64> = init[0]
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .fold(BTreeMap::new(), |mut acc, age| {
            *acc.entry(age).or_insert(0) += 1;
            acc
        });
    for day in 1..days {
        if timers.keys().nth(0) == Some(&day) {
            let cnt = timers.remove(&day).unwrap();
            *timers.entry(day + 6 + 1).or_insert(0) += cnt;
            *timers.entry(day + 8 + 1).or_insert(0) += cnt;
        }
    }
    let total = timers.iter().map(|ent| ent.1).sum::<u64>();
    total
}

pub fn part1(input: &Vec<String>) -> bool {
    let total = lanternfish(input, 80);
    total == 365862
}

pub fn part2(input: &Vec<String>) -> bool {
    let total = lanternfish(input, 256);
    total == 1653250886439
}
