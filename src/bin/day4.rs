use std::{env, io};
use std::collections::{HashMap, HashSet, VecDeque};
use advent_code_lib::{all_lines, ExNihilo, MultiLineObjects};
use histogram::Histogram;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: day4 filename");
        Ok(())
    } else {
        let mut game = BingoGame::from_file(args[1].as_str())?;
        println!("Part 1 score: {}", game.next().unwrap());
        println!("Part 2 score: {}", game.last().unwrap());
        Ok(())
    }
}

#[derive(Clone)]
struct BingoGame {
    calls: VecDeque<usize>,
    boards: Vec<BingoBoard>
}

impl BingoGame {
    pub fn from_file(filename: &str) -> io::Result<Self> {
        let mut lines = all_lines(filename)?;
        let calls = lines.next().unwrap().split(",").map(|s| s.parse().unwrap()).collect();
        lines.next(); // Skip blank line
        let mut reader = MultiLineObjects::new();
        for line in lines {
            reader.add_line(line.as_str(), &mut |board: &mut BingoBoard, line| {
                board.add_row(line);
            });
        }
        Ok(BingoGame {calls, boards: reader.objects()})
    }

    pub fn next_round_score(&mut self) -> Option<usize> {
        self.boards.retain(|board| !board.is_winner());
        let play = self.calls.pop_front().unwrap();
        for board in self.boards.iter_mut() {
            board.mark(play);
        }
        self.boards.iter()
            .find(|board| board.is_winner())
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
    row_counts: Histogram<usize>,
    col_counts: Histogram<usize>,
    unmarked: HashSet<usize>
}

impl ExNihilo for BingoBoard {
    fn create() -> Self {
        BingoBoard {num2pos: HashMap::new(), row_counts: Histogram::new(),
            col_counts: Histogram::new(), num_cols: 0, num_rows: 0,
            unmarked: HashSet::new()}
    }
}

impl BingoBoard {
    fn add_row(&mut self, row: &str) {
        let row_num = self.num_rows;
        let mut col_num = 0;
        for num in row.split_whitespace() {
            let num = num.parse().unwrap();
            self.num2pos.insert(num, (col_num, row_num));
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
        });
    }

    fn winning(histogram: &Histogram<usize>, goal: usize) -> bool {
        histogram.mode().map_or(false, |(_,count)| count >= goal)
    }

    pub fn is_winner(&self) -> bool {
        BingoBoard::winning(&self.col_counts, self.num_cols) ||
            BingoBoard::winning(&self.row_counts, self.num_rows)
    }
}