use std::num::ParseIntError;

fn parse_lines(input: &Vec<String>) -> Result<Vec<u32>, ParseIntError> {
    input.iter().map(|s| Ok(s.parse()?)).collect()
}

pub fn part1(input: &Vec<String>) -> bool {
    let depths = parse_lines(input).unwrap();
    depths.windows(2).filter(|x| x[1] > x[0]).count() == 1766
}

pub fn part2(input: &Vec<String>) -> bool {
    let depths = parse_lines(input).unwrap();
    depths.windows(4).filter(|x| x[3] > x[0]).count() == 1797
}
