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

pub fn part2(_input: &Vec<String>) -> bool {
    todo!()
}
