use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(position: usize) -> Self {
        Self { position, score: 0 }
    }

    fn move_forward(&mut self, steps: usize) {
        let steps = steps % 10;
        self.position = (self.position - 1 + steps) % 10 + 1;
        self.score += self.position
    }
}

fn play<const N: usize>(players: &mut [Player; N]) -> (usize /* winner */, usize /* rolled */) {
    let mut dice = (1..101).cycle();
    for (turn, idx) in [0, 1 as usize].iter().cycle().enumerate() {
        let steps = dice.next().unwrap() + dice.next().unwrap() + dice.next().unwrap();
        players[*idx].move_forward(steps);
        if players[*idx].score >= 1000 {
            return (*idx, (turn + 1) * 3);
        }
    }
    unreachable!("there will be a winner!")
}

pub fn part1(_input: &Vec<String>) -> bool {
    let mut players = [Player::new(10), Player::new(9)];

    let (winner, rolled) = play(&mut players);

    let loser = &players[(winner + 1) % 2];

    let res = loser.score * rolled;

    res == 918081
}

#[allow(unused)]
fn play_quantum<const N: usize>(players: &[Player; N]) -> [usize; N] {
    let mut queue = VecDeque::new();
    queue.push_back((0, 1, players.clone()));

    let mut counter = [0; N];
    while let Some((idx, universe, players)) = queue.pop_front() {
        for (steps, n) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
            let mut players = players.clone();
            players[idx].move_forward(steps);
            if players[idx].score >= 21 {
                counter[idx] += n * universe;
            } else {
                queue.push_back(((idx + 1) % N, n * universe, players))
            }
        }
    }
    counter
}

pub fn part2(_input: &Vec<String>) -> bool {
    let players = [Player::new(10), Player::new(9)];
    let counter = play_quantum(&players);

    let res = counter.into_iter().max().unwrap();

    res == 158631174219251
}
