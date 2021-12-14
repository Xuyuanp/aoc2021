pub fn part1(input: &Vec<String>) -> bool {
    let mut positions: Vec<i32> = input[0]
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let len = positions.len();
    let median = *positions.select_nth_unstable(len / 2).1;
    let fuel = positions.iter().map(|p| (median - p).abs()).sum::<i32>();
    fuel == 349812
}

pub fn part2(input: &Vec<String>) -> bool {
    let positions: Vec<i32> = input[0]
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mean: i32 = positions.iter().sum::<i32>() / (positions.len() as i32);
    let fuel = positions
        .iter()
        .map(|p| {
            let d = (p - mean).abs();
            d * (d + 1) / 2
        })
        .sum::<i32>();
    fuel == 99763899
}
