use std::collections::BinaryHeap;

pub fn part1(input: &Vec<String>) -> bool {
    let mut timers: BinaryHeap<i32> = input[0]
        .split(",")
        .map(|s| -s.parse::<i32>().unwrap())
        .collect();
    for day in 1..80 {
        while timers.peek() == Some(&(-&day)) {
            timers.pop();
            timers.push(-day - 6 - 1);
            timers.push(-day - 8 - 1);
        }
    }
    timers.len() == 365862
}

pub fn part2(_input: &Vec<String>) -> bool {
    false
}
