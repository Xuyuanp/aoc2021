use std::num::ParseIntError;
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Number {
    Regular(u32),
    Pair(Box<Number>, Box<Number>),
}

#[derive(Debug, PartialEq, Eq)]
enum ParseError {
    ShortRead,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let new = Self::new_pair(self, rhs);
        new.reduce()
    }
}

impl FromStr for Number {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s
            .replace("[", "[ ")
            .replace("]", " ]")
            .replace(",", " ")
            .split(" ")
            .map(String::from)
            .collect::<Vec<_>>();

        Self::parse_one(&mut tokens.into_iter())
    }
}

impl Number {
    fn new_pair(lhs: Self, rhs: Self) -> Self {
        Self::Pair(Box::new(lhs), Box::new(rhs))
    }

    fn parse_one(tokens: &mut dyn Iterator<Item = String>) -> Result<Self, ParseError> {
        match tokens.next().ok_or(ParseError::ShortRead)?.as_str() {
            "[" => {
                let lhs = Self::parse_one(tokens)?;
                let rhs = Self::parse_one(tokens)?;
                assert_eq!(
                    "]",
                    tokens.next().ok_or(ParseError::ShortRead)?,
                    "leak of ']'"
                );
                Ok(Self::new_pair(lhs, rhs))
            }
            "]" => unreachable!("stale ']'"),
            s => Ok(Self::Regular(s.parse()?)),
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Self::Regular(x) => *x,
            Self::Pair(lhs, rhs) => lhs.magnitude() * 3 + rhs.magnitude() * 2,
        }
    }

    fn reduce(mut self) -> Self {
        loop {
            if self.try_explode(0).0 {
                continue;
            }
            if self.try_split() {
                continue;
            }
            break;
        }
        self
    }

    fn try_explode(&mut self, depth: usize) -> (bool, Option<u32>, Option<u32>) {
        match self {
            Self::Regular(_) => (false, None, None),
            Self::Pair(lhs, rhs) => {
                let lhs_ref: &Number = lhs;
                let rhs_ref: &Number = rhs;
                if let (Self::Regular(l), Self::Regular(r)) = (lhs_ref, rhs_ref) {
                    if depth >= 4 {
                        let (l, r) = (*l, *r);
                        *self = Self::Regular(0);
                        return (true, Some(l), Some(r));
                    }
                };
                let (result, left, mut right) = lhs.try_explode(depth + 1);
                if result {
                    if right.is_some() {
                        rhs.add_left(right.unwrap());
                        right = None
                    }
                    return (true, left, right);
                }
                let (result, mut left, right) = rhs.try_explode(depth + 1);
                if result {
                    if left.is_some() {
                        lhs.add_right(left.unwrap());
                        left = None
                    }
                    return (true, left, right);
                }
                (false, None, None)
            }
        }
    }

    fn try_split(&mut self) -> bool {
        match self {
            Self::Regular(x) => {
                if *x >= 10 {
                    let new_left = *x / 2;
                    let new_right = *x - new_left;
                    *self = Self::Pair(
                        Box::new(Self::Regular(new_left)),
                        Box::new(Self::Regular(new_right)),
                    );
                    true
                } else {
                    false
                }
            }
            Self::Pair(lhs, rhs) => lhs.try_split() || rhs.try_split(),
        }
    }

    fn add_left(&mut self, num: u32) {
        match self {
            Self::Regular(x) => *x += num,
            Self::Pair(lhs, _) => lhs.add_left(num),
        }
    }

    fn add_right(&mut self, num: u32) {
        match self {
            Self::Regular(x) => *x += num,
            Self::Pair(_, rhs) => rhs.add_right(num),
        }
    }
}

pub fn part1(input: &Vec<String>) -> bool {
    let mut numbers = input.iter().map(|line| line.parse::<Number>().unwrap());
    let first = numbers.next().unwrap();
    let total = numbers.fold(first, |acc, num| acc + num);
    let res = total.magnitude();
    res == 4057
}

pub fn part2(input: &Vec<String>) -> bool {
    let numbers: Vec<_> = input
        .iter()
        .map(|line| line.parse::<Number>().unwrap())
        .collect();
    let mut res = u32::MIN;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }
            let (x, y) = (numbers[i].clone(), numbers[j].clone());
            res = res.max((x + y).magnitude());
        }
    }
    res == 4683
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "[1,2]".parse(),
            Ok(Number::Pair(
                Box::new(Number::Regular(1)),
                Box::new(Number::Regular(2))
            )),
            ""
        );

        assert_eq!(
            "[1,[2,3]]".parse(),
            Ok(Number::Pair(
                Box::new(Number::Regular(1)),
                Box::new(Number::Pair(
                    Box::new(Number::Regular(2)),
                    Box::new(Number::Regular(3)),
                ))
            )),
            ""
        );
        assert_eq!(
            "[[1,2],3]".parse(),
            Ok(Number::Pair(
                Box::new(Number::Pair(
                    Box::new(Number::Regular(1)),
                    Box::new(Number::Regular(2)),
                )),
                Box::new(Number::Regular(3)),
            )),
        );

        assert_eq!(
            "[[1,2],[3,4]]".parse(),
            Ok(Number::Pair(
                Box::new(Number::Pair(
                    Box::new(Number::Regular(1)),
                    Box::new(Number::Regular(2)),
                )),
                Box::new(Number::Pair(
                    Box::new(Number::Regular(3)),
                    Box::new(Number::Regular(4)),
                ))
            )),
        );
    }

    #[test]
    fn test_add() {
        let num1: Number = "[1,2]".parse().unwrap();
        let num2: Number = "[3,4]".parse().unwrap();
        let num3: Number = "[[1,2],[3,4]]".parse().unwrap();
        assert_eq!(num1 + num2, num3);
    }

    #[test]
    fn test_reduce() {
        let old: Number = "[[[[[9,8],1],2],3],4]".parse().unwrap();
        let new = old.reduce();
        let expect = "[[[[0,9],2],3],4]".parse().unwrap();
        assert_eq!(new, expect);

        let old: Number = "[7,[6,[5,[4,[3,2]]]]]".parse().unwrap();
        let new = old.reduce();
        let expect = "[7,[6,[5,[7,0]]]]".parse().unwrap();
        assert_eq!(new, expect);

        let old: Number = "[[6,[5,[4,[3,2]]]],1]".parse().unwrap();
        let new = old.reduce();
        let expect = "[[6,[5,[7,0]]],3]".parse().unwrap();
        assert_eq!(new, expect);

        let old: Number = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap();
        let new = old.reduce();
        let expect = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".parse().unwrap();
        assert_eq!(new, expect);
    }
}
