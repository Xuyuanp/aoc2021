use std::collections::BTreeMap;

fn lanternfish(init: &Vec<String>, days: u32) -> u64 {
    let mut timers: BTreeMap<u32, u64> = init[0]
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .fold(BTreeMap::new(), |mut acc, age| {
            let cnt = acc.remove(&age).unwrap_or(0);
            acc.insert(age, cnt + 1);
            acc
        });
    for day in 1..days {
        if timers.keys().nth(0) == Some(&day) {
            let cnt = timers.remove(&day).unwrap();

            let cnt1 = timers.remove(&(day + 6 + 1)).unwrap_or(0);
            timers.insert(day + 6 + 1, cnt + cnt1);

            let cnt2 = timers.remove(&(day + 8 + 1)).unwrap_or(0);
            timers.insert(day + 8 + 1, cnt + cnt2);
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
