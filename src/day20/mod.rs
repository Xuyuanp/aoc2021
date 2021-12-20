use std::convert::TryInto;

fn enhance(image: &Vec<Vec<char>>, trans: &[char; 512], default: char) -> (Vec<Vec<char>>, char) {
    let (m, n) = (image.len(), image[0].len());
    let mut new: Vec<Vec<char>> = Vec::with_capacity(m + 2);
    for _ in 0..m + 2 {
        let row = ".".repeat(n + 2).chars().collect();
        new.push(row);
    }

    let (m, n) = (m as i32, n as i32);
    (-1..m + 1)
        .enumerate()
        .map(move |x| (-1..n + 1).enumerate().map(move |y| (x, y)))
        .flatten()
        .for_each(|p| {
            let ((new_x, old_x), (new_y, old_y)) = p;
            let idx = (-1..2)
                .map(move |dx| (-1..2).map(move |dy| (old_x + dx, old_y + dy)))
                .flatten()
                .map(|(x, y)| {
                    if x < 0 || y < 0 || x >= m || y >= n {
                        default
                    } else {
                        image[x as usize][y as usize]
                    }
                })
                .fold(0, |acc, c| {
                    acc << 1
                        | (match c {
                            '.' => 0,
                            '#' => 1,
                            _ => unreachable!("unknown pixel: {}", c),
                        } as usize)
                });
            new[new_x][new_y] = trans[idx];
        });

    (new, if default == '.' { '#' } else { '.' })
}

pub fn part1(input: &Vec<String>) -> bool {
    let trans: [char; 512] = input[0].chars().collect::<Vec<_>>().try_into().unwrap();
    let mut image: Vec<Vec<char>> = input[2..]
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect();

    let mut default = '.';
    for _ in 0..2 {
        let new = enhance(&image, &trans, default);
        image = new.0;
        default = new.1;
    }

    let res = image.iter().flatten().filter(|c| c == &&'#').count();
    res == 5306
}

pub fn part2(input: &Vec<String>) -> bool {
    let trans: [char; 512] = input[0].chars().collect::<Vec<_>>().try_into().unwrap();
    let mut image: Vec<Vec<char>> = input[2..]
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect();

    let mut default = '.';
    for _ in 0..50 {
        let new = enhance(&image, &trans, default);
        image = new.0;
        default = new.1;
    }

    let res = image.iter().flatten().filter(|c| c == &&'#').count();
    res == 17497
}
