use std::collections::HashMap;
use std::convert::TryInto;

pub fn part1(input: &Vec<String>) -> bool {
    let cnt = input
        .iter()
        .map(|line| {
            line.split("|")
                .nth(1)
                .unwrap()
                .trim()
                .split_whitespace()
                .filter(|s| {
                    let l = s.len();
                    l == 2 || l == 3 || l == 4 || l == 7
                })
                .count()
        })
        .sum::<usize>();
    cnt == 479
}

fn segment2bits(segment: &String) -> u32 {
    let base = 'a' as u32;
    let bits = segment
        .chars()
        .fold(0, |acc, c| acc | (1 << (c as u32 - base)));

    bits
}

fn parse_line(line: &String) -> ([u32; 10], [u32; 4]) {
    let seps: [String; 2] = line
        .split(" | ")
        .map(String::from)
        .collect::<Vec<String>>()
        .try_into()
        .unwrap();

    let mut signals = seps[0]
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();

    signals.sort_by_key(String::len);

    let signals: [u32; 10] = signals
        .iter()
        .map(segment2bits)
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap();

    let output: [u32; 4] = seps[1]
        .split_whitespace()
        .map(String::from)
        .map(|s| segment2bits(&s))
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap();

    (signals, output)
}

fn decode(signals: [u32; 10]) -> HashMap<u32, u32> {
    // ["bc", "bfc", "bdcg", "gfaeb", "cgbfe", "fcdge", "cfeabd", "efcagd", "dcfgbe", "bgadfce"]
    //   2      3      4        5        5        5        6          6         6         7
    //   0      1      2        3        4        5        6          7         8         9
    //
    // 2: 1
    // 3: 7
    // 4: 4
    // 5: 2, 3, 5
    // 6: 0, 6, 9
    // 7: 8
    let mut map = HashMap::new();

    let (one, four, seven, eight) = (signals[0], signals[2], signals[1], signals[9]);

    // one | six == eight
    let six = [6, 7, 8]
        .iter()
        .map(|i| signals[*i])
        .filter(|n| n | one == eight)
        .next()
        .unwrap();

    // zero | four == eight
    let zero = [6, 7, 8]
        .iter()
        .map(|i| signals[*i])
        .filter(|n| n != &six)
        .filter(|n| n | four == eight)
        .next()
        .unwrap();

    let nine = [6, 7, 8]
        .iter()
        .map(|i| signals[*i])
        .filter(|n| n != &six && n != &zero)
        .next()
        .unwrap();

    // two & four == eight
    let two = [3, 4, 5]
        .iter()
        .map(|i| signals[*i])
        .filter(|n| n | four == eight)
        .next()
        .unwrap();

    // three & seven == seven
    let three = [3, 4, 5]
        .iter()
        .map(|i| signals[*i])
        .filter(|n| n & seven == seven)
        .next()
        .unwrap();

    // five | one == nine
    let five = [3, 4, 5]
        .iter()
        .map(|i| signals[*i])
        .filter(|n| n != &two && n != &three)
        .next()
        .unwrap();

    map.insert(zero, 0);
    map.insert(one, 1);
    map.insert(two, 2);
    map.insert(three, 3);
    map.insert(four, 4);
    map.insert(five, 5);
    map.insert(six, 6);
    map.insert(seven, 7);
    map.insert(eight, 8);
    map.insert(nine, 9);

    map
}

fn convert(map: &HashMap<u32, u32>, output: [u32; 4]) -> u32 {
    output
        .iter()
        .fold(0, |acc, v| acc * 10 + map.get(v).unwrap())
}

pub fn part2(input: &Vec<String>) -> bool {
    let res: u32 = input
        .iter()
        .map(parse_line)
        .map(|(signals, output)| {
            let map = decode(signals);
            convert(&map, output)
        })
        .sum();
    res == 1041746
}
