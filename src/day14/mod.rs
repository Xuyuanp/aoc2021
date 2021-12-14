use std::collections::HashMap;

fn parse_input(input: &Vec<String>) -> (String, HashMap<String, char>) {
    let mut map = HashMap::new();

    let templ = input.iter().next().unwrap().to_string();

    input[2..].iter().for_each(|line| {
        let seps: Vec<&str> = line.split(" -> ").collect();
        map.insert(seps[0].to_string(), seps[1].chars().next().unwrap());
    });

    (templ, map)
}

pub fn part1(input: &Vec<String>) -> bool {
    let (templ, map) = parse_input(input);

    let mut start: Vec<char> = templ.chars().collect();

    for _i in 0..10 {
        start = start.windows(2).map(|chars| {
            let pair: String = chars.iter().collect();
            [chars[0], *map.get(&pair).unwrap(), chars[1]]
        }).fold(Vec::new(), |mut vec, chars| {
            if vec.len() == 0 {
                vec.extend_from_slice(&chars);
            } else {
                vec.extend_from_slice(&chars[1..])
            }
            vec
        }) ;
    }
    let counter = start.iter().fold(HashMap::new(), |mut counter, ch| {
        *counter.entry(ch).or_insert(0) += 1;
        counter
    });

    let mut freq: Vec<i32> = counter.into_values().collect();
    let len = freq.len();
    freq.select_nth_unstable(0);
    freq.select_nth_unstable(len-1);

    let res = freq[len-1] - freq[0];

    res == 2112
}

pub fn part2(_input: &Vec<String>) -> bool {
    unimplemented!();
}
