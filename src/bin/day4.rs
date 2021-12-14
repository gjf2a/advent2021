use std::io;
use std::collections::{HashMap, HashSet, VecDeque};
use advent_code_lib::{all_lines, ExNihilo, advent_main, line2numbers_iter, MultiLineObjects};
use hash_histogram::HashHistogram;

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        let mut game = BingoGame::from_file(args[1].as_str())?;
        println!("Part 1 score: {}", game.next().unwrap());
        println!("Part 2 score: {}", game.last().unwrap());
        Ok(())
    })
}

#[derive(Clone)]
struct BingoGame {
    calls: VecDeque<usize>,
    boards: Vec<BingoBoard>
}

impl BingoGame {
    pub fn from_file(filename: &str) -> io::Result<Self> {
        let mut lines = all_lines(filename)?;
        let calls = line2numbers_iter(lines.next().unwrap().as_str()).collect();
        lines.next(); // Skip blank line
        let boards = MultiLineObjects::from_iterator(lines, |board: &mut BingoBoard, line| {
            board.add_row(line);
        }).objects();
        Ok(BingoGame {calls, boards})
    }

    pub fn next_round_score(&mut self) -> Option<usize> {
        self.boards.retain(|board| !board.winner);
        let play = self.calls.pop_front().unwrap();
        for board in self.boards.iter_mut() {
            board.mark(play);
        }
        self.boards.iter()
            .find(|board| board.winner)
            .map(|board| board.unmarked.iter().sum::<usize>() * play)
    }

    pub fn num_boards(&self) -> usize {
        self.boards.len()
    }
}

impl Iterator for BingoGame {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num_boards() == 1 {
            None
        } else {
            loop {
                if let Some(score) = self.next_round_score() {
                    return Some(score);
                }
            }
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
struct BingoBoard {
    num2pos: HashMap<usize,(usize,usize)>,
    num_rows: usize,
    num_cols: usize,
    row_counts: HashHistogram<usize>,
    col_counts: HashHistogram<usize>,
    unmarked: HashSet<usize>,
    winner: bool
}

impl ExNihilo for BingoBoard {
    fn create() -> Self {
        BingoBoard {num2pos: HashMap::new(), row_counts: HashHistogram::new(),
            col_counts: HashHistogram::new(), num_cols: 0, num_rows: 0,
            unmarked: HashSet::new(), winner: false}
    }
}

impl BingoBoard {
    fn add_row(&mut self, row: &str) {
        let mut col_num = 0;
        for num in row.split_whitespace() {
            let num = num.parse().unwrap();
            self.num2pos.insert(num, (col_num, self.num_rows));
            self.unmarked.insert(num);
            col_num += 1;
        }
        self.num_rows += 1;
        if col_num > self.num_cols {
            self.num_cols = col_num;
        }
    }

    pub fn mark(&mut self, num: usize) {
        self.unmarked.remove(&num);
        self.num2pos.get(&num).map(|(col, row)| {
            self.col_counts.bump(col);
            self.row_counts.bump(row);
            self.winner = self.winner ||
                self.col_counts.count(col) == self.num_cols ||
                self.row_counts.count(row) == self.num_rows;
        });
    }
}