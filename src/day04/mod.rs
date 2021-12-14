use std::collections::HashSet;
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
        (0..5 as usize)
            .map(move |i| (0..5 as usize).map(move |j| (i, j)))
            .flatten()
            .filter(|(i, j)| self.rows[*i].cols[*j] == val)
            .next()
            .map_or(false, |(i, j)| {
                self.rows[i].cols[j] -= 1000;
                self.rows[i].cols.iter().filter(|x| x >= &&0).count() == 0
                    || self
                        .rows
                        .iter()
                        .map(|row| row.cols[j])
                        .filter(|x| x >= &0)
                        .count()
                        == 0
            })
    }

    fn score(&self, num: i32) -> i32 {
        let sum_of_unmarked: i32 = self
            .rows
            .iter()
            .map(|row| row.cols.iter().filter(|x| x >= &&0).sum::<i32>())
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

    fn play2(&mut self) -> (i32, usize) {
        let mut boards: HashSet<usize> = (0..self.boards.len()).collect();
        for num in self.nums.iter() {
            for idx in boards.clone() {
                if self.boards[idx].mark(*num) {
                    if boards.len() == 1 {
                        return (*num, idx);
                    } else {
                        boards.remove(&idx);
                    }
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

pub fn part2(input: &Vec<String>) -> bool {
    let mut bingo = Bingo::new(input);
    let (num, winner) = bingo.play2();

    let res = bingo.boards[winner].score(num);

    res == 1830
}
