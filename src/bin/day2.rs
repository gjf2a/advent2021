use std::{env, io};
use advent_code_lib::{for_each_line, Position};

struct Submarine {
    pos: Position,
    aim: isize
}

impl Submarine {
    pub fn new() -> Self {
        Submarine {pos: Position::new(), aim: 0}
    }

    pub fn report(&self) {
        println!("Final position: horizontal: {}, depth: {}, product: {}",
                 self.pos.col, self.pos.row, self.pos.col * self.pos.row);
    }

    pub fn update_1(&mut self, command: &str, distance: isize) {
        self.pos += match command {
            "forward" => Position::from((distance, 0)),
            "down" => Position::from((0, distance)),
            "up" => Position::from((0, -distance)),
            other => panic!("Illegible input: {}", other)
        };
    }

    pub fn update_2(&mut self, command: &str, distance: isize) {
        match command {
            "forward" => self.pos += Position::from((distance, distance * self.aim)),
            "down" => self.aim += distance,
            "up" => self.aim -= distance,
            other => panic!("Illegible input: {}", other)
        };
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: day2 inputfile (1|2)");
    } else {
        let mut sub = Submarine::new();
        for_each_line(args[1].as_str(), |line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let distance = parts[1].parse::<isize>().unwrap();
            match args[2].as_str() {
                "1" => sub.update_1(parts[0], distance),
                "2" => sub.update_2(parts[0], distance),
                other => {println!("Illegal argument: {}", other);}
            }
            Ok(())
        })?;
        sub.report();
    }
    Ok(())
}