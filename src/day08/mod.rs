pub fn part1(input: &Vec<String>) -> bool {
    let cnt = input.iter().map(|line| {
        line.split("|").nth(1).unwrap().trim().split_whitespace().filter(|s| {
            let l = s.len();
            l == 2 || l == 3 || l == 4 || l == 7
        }).count()
    }).sum::<usize>();
    cnt == 479
}

pub fn part2(_input: &Vec<String>) -> bool {
    false
}
