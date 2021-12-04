pub fn part1(input: &Vec<String>) -> bool {
    let (gamma, epsilon) = input
        .iter()
        .fold([0; 12] as [i32; 12], |mut acc, s| {
            for (i, c) in s.chars().enumerate() {
                match c {
                    '1' => acc[i] += 1,
                    '0' => acc[i] -= 1,
                    _ => panic!("unknown char"),
                }
            }
            acc
        })
        .iter()
        .fold((0 as u32, 0 as u32), |(gamma, epsilon), cnt| {
            let most_common = if cnt > &0 { 1 } else { 0 };
            (gamma << 1 | most_common, epsilon << 1 | (most_common ^ 1))
        });
    gamma * epsilon == 2498354
}

fn get_rate(input: &Vec<String>, idx: usize, most: bool) -> Vec<String> {
    let cnt: i32 = input
        .iter()
        .map(|s| match s.chars().nth(idx).unwrap() {
            '1' => 1,
            '0' => -1,
            _ => panic!("fuck"),
        })
        .sum();

    let filtered: Vec<String> = input
        .iter()
        .filter(|s| {
            let c = s.chars().nth(idx).unwrap();
            c == '1' && cnt >= 0 && most
                || c == '0' && cnt < 0 && most
                || c == '1' && cnt < 0 && !most
                || c == '0' && cnt >= 0 && !most
        })
        .map(String::clone)
        .collect();

    if filtered.len() == 1 {
        filtered
    } else {
        get_rate(&filtered, idx + 1, most)
    }
}

fn bits_to_int(bits: &String) -> u32 {
    bits.chars().fold(0, |acc, c| {
        acc << 1
            | match c {
                '0' => 0,
                '1' => 1,
                _ => panic!("fuck"),
            }
    })
}

pub fn part2(input: &Vec<String>) -> bool {
    let oxs = get_rate(input, 0, true);
    let co2s = get_rate(input, 0, false);

    let ox = bits_to_int(oxs.first().unwrap());
    let co2 = bits_to_int(co2s.first().unwrap());

    ox * co2 == 3277956
}
