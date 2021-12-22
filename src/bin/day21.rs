use std::collections::HashMap;
use std::io;
use std::ops::Add;
use advent_code_lib::{advent_main, all_lines, ExNihilo};
use bare_metal_modulo::{MNum, ModNumC};

const DIE_FACES_1: usize = 100;
const DIE_FACES_2: usize = 3;
const BOARD_SQUARES: usize = 10;
const ROLLS_PER_TURN: usize = 3;
const NUM_PLAYERS: usize = 2;
const TARGET_SCORE_1: u128 = 1000;
const TARGET_SCORE_2: u128 = 21;

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        let mut game = part_1_game(args[1].as_str())?;
        game.play_until_completion();
        println!("Part 1: {}", game.part_1_score());
        println!("Part 2: {}", AllGamesFrom::part_2(args[1].as_str())?);
        Ok(())
    })
}

/////////////////
// Part 1 Code //
/////////////////

fn part_1_game(filename: &str) -> io::Result<Game<DeterministicDie<DIE_FACES_1>, TARGET_SCORE_1>> {
    Game::from_file(filename)
}

#[derive(Debug, Copy, Clone)]
struct Game<D, const G: u128> {
    players: [Player; NUM_PLAYERS],
    current_player: ModNumC<usize, NUM_PLAYERS>,
    die: D,
    num_rolls: u128
}

fn grab_nums(filename: &str) -> io::Result<[ModNumC<u128, BOARD_SQUARES>; NUM_PLAYERS]> {
    let mut nums = [ModNumC::new(0), ModNumC::new(0)];
    all_lines(filename)?.enumerate().for_each(|(i, line)| {nums[i] = line_num(line.as_str());});
    Ok(nums)
}

impl <D:Copy + Iterator<Item=u128> + ExNihilo, const G: u128> Game<D, G> {
    fn from_file(filename: &str) -> io::Result<Self> {
        Ok(Self::new(all_lines(filename)?, D::create()))
    }

    fn new<I:Iterator<Item=String>>(mut lines: I, die: D) -> Self {
        let player1 = Player::new(lines.next().unwrap().as_str());
        let player2 = Player::new(lines.next().unwrap().as_str());
        Game {players: [player1, player2], current_player: ModNumC::new(0), die, num_rolls: 0}
    }

    fn roll(&mut self) -> u128 {
        let mut total = 0;
        for _ in 0..ROLLS_PER_TURN {
            total += self.die.next().unwrap();
        }
        total
    }

    fn mover(&mut self) -> &mut Player {
        &mut self.players[self.current_player.a()]
    }

    fn play_until_completion(&mut self) {
        loop {
            let distance = self.roll();
            self.mover().play_one_move(distance);
            self.num_rolls += ROLLS_PER_TURN as u128;
            if self.mover().total_score() >= G {
                break;
            } else {
                self.current_player += 1;
            }
        }
    }

    fn part_1_score(&self) -> u128 {
        self.players.iter().map(|p| p.total_score()).min().unwrap() * self.num_rolls
    }
}

#[derive(Copy, Clone, Debug)]
struct DeterministicDie<const F: usize> {
    face: ModNumC<u128, F>
}

impl <const F: usize> Iterator for DeterministicDie<F> {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.face.a() + 1;
        self.face += 1;
        Some(result)
    }
}

impl <const F: usize> ExNihilo for DeterministicDie<F> {
    fn create() -> Self {
        DeterministicDie {face: ModNumC::new(0)}
    }
}

#[derive(Copy, Clone, Debug)]
struct Player {
    position: ModNumC<u128, BOARD_SQUARES>,
    position_sum: u128,
    moves: u128
}

fn line_num(line: &str) -> ModNumC<u128, BOARD_SQUARES> {
    ModNumC::new(line.split_whitespace().last().unwrap().parse::<u128>().unwrap() - 1)
}

impl Player {
    fn new(start: &str) -> Self {
        Player {position_sum: 0, moves: 0, position: line_num(start)}
    }

    fn space_at(&self) -> u128 {
        self.position.a() + 1
    }

    fn total_score(&self) -> u128 {
        self.position_sum + self.moves
    }

    fn play_one_move(&mut self, distance: u128) {
        self.position += distance;
        self.moves += 1;
        self.position_sum += self.position.a();
    }
}

/////////////////
// Part 2 Code //
/////////////////

struct AllGamesFrom {
    wins_from: HashMap<GameKey, WinnerTally>,
    roller: DiracRoller
}

impl AllGamesFrom {
    fn part_2(filename: &str) -> io::Result<u128> {
        Ok(Self::max_wins(grab_nums(filename)?))
    }

    fn new() -> Self {
        AllGamesFrom {wins_from: HashMap::new(), roller: DiracRoller::new()}
    }

    fn max_wins(start: [ModNumC<u128, BOARD_SQUARES>; NUM_PLAYERS]) -> u128 {
        let mut games = AllGamesFrom::new();
        let wins = games.get_wins_for(GameKey {locations: start, scores: [0, 0], current: ModNumC::new(0)});
        println!("Distinct games: {}", games.wins_from.len());
        wins.winner_count()
    }

    fn get_wins_for(&mut self, key: GameKey) -> WinnerTally {
        match self.wins_from.get(&key) {
            Some(wins) => *wins,
            None => {
                let tally = match key.winner() {
                    None => {
                        let mut tally = WinnerTally::new();
                        for total_roll in self.roller.rolls() {
                            tally = tally + self.get_wins_for(key.moved_by(total_roll as u128));
                        }
                        tally
                    }
                    Some(winner) => winner
                };
                self.wins_from.insert(key, tally);
                tally
            }
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct GameKey {
    locations: [ModNumC<u128, BOARD_SQUARES>; NUM_PLAYERS],
    scores: [u128; NUM_PLAYERS],
    current: ModNumC<usize, NUM_PLAYERS>
}

impl GameKey {
    fn winner(&self) -> Option<WinnerTally> {
        self.scores.iter().enumerate()
            .find(|(_, s)| **s >= TARGET_SCORE_2)
            .map(|(p, _)| {
                let mut result = WinnerTally::new();
                result.tally[p] = 1;
                result
            })
    }

    fn moved_by(&self, roll: u128) -> GameKey {
        let mut next = self.clone();
        next.locations[self.current.a()] += roll;
        next.scores[self.current.a()] += next.locations[self.current.a()].a() + 1;
        next.current += 1;
        next
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct WinnerTally {
    tally: [u128; NUM_PLAYERS]
}

impl WinnerTally {
    fn new() -> Self {
        WinnerTally {tally:[0, 0]}
    }

    fn winner_count(&self) -> u128 {
        self.tally.iter().max().copied().unwrap()
    }
}

impl Add for WinnerTally {
    type Output = WinnerTally;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = WinnerTally::new();
        self.tally.iter().zip(rhs.tally.iter()).enumerate()
            .for_each(|(i, (a, b))| result.tally[i] += a + b);
        result
    }
}

struct DiracRoller {
    rolls: Vec<u128>
}

impl DiracRoller {
    fn new() -> Self {
        let mut rolls = Vec::new();
        for roll1 in 1..=DIE_FACES_2 {
            for roll2 in 1..=DIE_FACES_2 {
                for roll3 in 1..=DIE_FACES_2 {
                    rolls.push((roll1 + roll2 + roll3) as u128);
                }
            }
        }
        assert_eq!(rolls.len(), DIE_FACES_2.pow(ROLLS_PER_TURN as u32));
        DiracRoller {rolls}
    }

    fn rolls(&self) -> Vec<u128> {
        self.rolls.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let mut game = part_1_game("ex/day21.txt").unwrap();
        game.play_until_completion();
        println!("{:?}", game);
        assert_eq!(game.players[0].total_score(), 1000);
        assert_eq!(game.players[0].space_at(), 10);
        assert_eq!(game.players[1].total_score(), 745);
        assert_eq!(game.players[1].space_at(), 3);
        assert_eq!(game.num_rolls, 993);
        assert_eq!(game.part_1_score(), 739785);
    }

    #[test]
    fn test_example_part_2() {
        let result = AllGamesFrom::part_2("ex/day21.txt").unwrap();
        assert_eq!(result, 444356092776315);
    }
}
