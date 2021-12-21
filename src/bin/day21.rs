use std::io;
use advent_code_lib::{advent_main, all_lines};
use bare_metal_modulo::{MNum, ModNumC};

const DIE_FACES: usize = 100;
const BOARD_SQUARES: usize = 10;
const ROLLS_PER_TURN: usize = 3;
const NUM_PLAYERS: usize = 2;

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        let mut game = part_1_game(args[1].as_str())?;
        game.play_until_completion();
        println!("Part 1: {}", game.part_1_score());
        Ok(())
    })
}

fn part_1_game(filename: &str) -> io::Result<Game<DeterministicDie>> {
    Ok(Game::new(all_lines(filename)?, DeterministicDie::new()))
}

#[derive(Debug)]
struct Game<D> {
    players: [Player; NUM_PLAYERS],
    current_player: ModNumC<usize, NUM_PLAYERS>,
    die: D,
    num_rolls: u64
}

impl <D:Copy + Iterator<Item=u64>> Game<D> {
    fn new<I:Iterator<Item=String>>(mut lines: I, die: D) -> Self {
        let player1 = Player::new(lines.next().unwrap().as_str());
        let player2 = Player::new(lines.next().unwrap().as_str());
        Game {players: [player1, player2], current_player: ModNumC::new(0), die, num_rolls: 0}
    }

    fn roll(&mut self) -> u64 {
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
            self.num_rolls += ROLLS_PER_TURN as u64;
            if self.mover().score >= 1000 {
                break;
            } else {
                self.current_player += 1;
            }
        }
    }

    fn part_1_score(&self) -> u64 {
        self.players.iter().map(|p| p.score).min().unwrap() * self.num_rolls
    }
}

#[derive(Copy, Clone, Debug)]
struct DeterministicDie {
    face: ModNumC<u64, DIE_FACES>
}

impl Iterator for DeterministicDie {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.face.a() + 1;
        self.face += 1;
        Some(result)
    }
}

impl DeterministicDie {
    fn new() -> Self {
        DeterministicDie {face: ModNumC::new(0)}
    }
}

#[derive(Debug)]
struct Player {
    position: ModNumC<u64, BOARD_SQUARES>,
    score: u64
}

impl Player {
    fn new(start: &str) -> Self {
        Player {score: 0, position: ModNumC::new(start.split_whitespace().last().unwrap().parse::<u64>().unwrap() - 1)}
    }

    fn space_at(&self) -> u64 {
        self.position.a() + 1
    }

    fn play_one_move(&mut self, distance: u64) {
        self.position += distance;
        self.score += self.space_at();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let mut game = part_1_game("ex/day21.txt").unwrap();
        game.play_until_completion();
        assert_eq!(game.players[0].score, 1000);
        assert_eq!(game.players[0].space_at(), 10);
        assert_eq!(game.players[1].score, 745);
        assert_eq!(game.players[1].space_at(), 3);
        assert_eq!(game.num_rolls, 993);
        assert_eq!(game.part_1_score(), 739785);
    }
}