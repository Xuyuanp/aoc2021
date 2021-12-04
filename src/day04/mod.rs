use std::convert::TryInto;

#[derive(Debug, Clone, Copy)]
struct Row {
    cols: [i32; 5],
}

impl Row {
    fn new(line: &String) -> Self {
        let cols: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Self {
            cols: cols.try_into().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Board {
    rows: [Row; 5],
}

#[derive(Debug)]
struct Bingo {
    nums: Vec<i32>,
    boards: Vec<Board>,
}

impl Board {
    fn new(lines: &[String]) -> Self {
        let rows: Vec<Row> = lines.iter().map(Row::new).collect();
        Self {
            rows: rows.try_into().unwrap(),
        }
    }

    fn mark(&mut self, val: i32) -> bool {
        for i in 0..5 {
            for j in 0..5 {
                if self.rows[i].cols[j] == val {
                    self.rows[i].cols[j] -= 1000;
                    return self.rows[i].cols.iter().filter(|x| x >= &&0).count() == 0
                        || self
                            .rows
                            .iter()
                            .map(|row| row.cols[j])
                            .filter(|x| x >= &0)
                            .count()
                            == 0;
                }
            }
        }
        false
    }

    fn score(&self, num: i32) -> i32 {
        let sum_of_unmarked: i32 = self
            .rows
            .iter()
            .map(|row| row.cols.iter().filter(|x| **x >= 0).sum::<i32>())
            .sum();
        sum_of_unmarked * num
    }
}

impl Bingo {
    fn new(input: &Vec<String>) -> Self {
        let nums: Vec<i32> = input
            .first()
            .unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();

        let boards = input[1..]
            .chunks(6)
            .map(|rows| Board::new(&rows[1..]))
            .collect();

        Self { nums, boards }
    }

    fn play(&mut self) -> (i32, usize) {
        for num in self.nums.iter() {
            for idx in 0..self.boards.len() {
                if self.boards[idx].mark(*num) {
                    return (*num, idx);
                }
            }
        }
        panic!("unreachable")
    }
}

pub fn part1(input: &Vec<String>) -> bool {
    let mut bingo = Bingo::new(input);
    let (num, winner) = bingo.play();

    let res = bingo.boards[winner].score(num);

    res == 67716
}

pub fn part2(_input: &Vec<String>) -> bool {
    false
}
