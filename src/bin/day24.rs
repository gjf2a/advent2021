use std::collections::VecDeque;
use std::io;
use std::num::ParseIntError;
use std::str::FromStr;
use advent_code_lib::{advent_main, all_lines, make_inner_io_error, make_io_error};
use bare_metal_modulo::{MNum, ModNumC};
use itertools::Itertools;

const VAR_NAMES: [char; 4] = ['w', 'x', 'y', 'z'];
const NUM_VARS: usize = VAR_NAMES.len();
const MODEL_NUM_LEN: usize = 14;

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        Ok(())
    })
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct ALU {
    program: Vec<ALUInstruction>,
    data: [isize; NUM_VARS]
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum OpCode {
    Add, Mul, Div, Mod, Eql, Inp
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct ALUInstruction {
    op: OpCode,
    arg1: ModNumC<usize,NUM_VARS>,
    arg2: ALUArg2
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ALUArg2 {
    Var(ModNumC<usize,NUM_VARS>), Val(isize)
}

impl ALU {
    fn from_file(filename: &str) -> io::Result<Self> {
        let mut program = Vec::new();
        for line in all_lines(filename)? {
            program.push(line.parse()?);
        }
        Ok(ALU {program, data: [0; NUM_VARS]})
    }

    fn run(&mut self, inputs: &Vec<isize>) {
        let mut input_queue: VecDeque<isize> = inputs.iter().copied().collect();
        for instruction in self.program.iter() {
            let a = self.data[instruction.arg1.a()];
            let b = instruction.arg2.value(&self.data);
            self.data[instruction.arg1.a()] = match instruction.op {
                OpCode::Add => a + b,
                OpCode::Mul => a * b,
                OpCode::Div => a / b,
                OpCode::Mod => a % b,
                OpCode::Eql => if a == b {1} else {0},
                OpCode::Inp => input_queue.pop_front().unwrap()
            };
        }
    }

    fn reset(&mut self) {
        self.data = self.data.map(|_| 0);
    }
}

impl ALUArg2 {
    fn value(&self, data: &[isize; NUM_VARS]) -> isize {
        match self {
            ALUArg2::Var(v) => data[v.a()],
            ALUArg2::Val(v) => *v
        }
    }
}

impl FromStr for ALUInstruction {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect_vec();
        let arg1 = var_from(parts[1])?;
        if parts[0] == "inp" {
            Ok(ALUInstruction {op: OpCode::Inp, arg1, arg2: ALUArg2::Val(0)})
        } else {
            let arg2 = parts[2].parse::<ALUArg2>()?;
            match parts[0] {
                "add" => Ok(ALUInstruction {op: OpCode::Add, arg1, arg2}),
                "mul" => Ok(ALUInstruction {op: OpCode::Mul, arg1, arg2}),
                "div" => Ok(ALUInstruction {op: OpCode::Div, arg1, arg2}),
                "mod" => Ok(ALUInstruction {op: OpCode::Mod, arg1, arg2}),
                "eql" => Ok(ALUInstruction {op: OpCode::Eql, arg1, arg2}),
                other => make_io_error(format!("{} is not an instruction", other).as_str())
            }
        }
    }
}

impl FromStr for ALUArg2 {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<isize>() {
            Ok(v) => Ok(ALUArg2::Val(v)),
            Err(_) => Ok(ALUArg2::Var(var_from(s)?))
        }
    }
}

fn var_from(v: &str) -> io::Result<ModNumC<usize,NUM_VARS>> {
    if v.len() == 1 {
        let c = v.chars().next().unwrap();
        VAR_NAMES.iter().enumerate()
            .find(|(_, n)| c == **n)
            .map(|(i,_)| ModNumC::new(i as usize))
            .ok_or(make_inner_io_error("Not a variable"))
    } else {
        make_io_error("Too long for a variable")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let mut alu = ALU::from_file("ex/day24a.txt").unwrap();
        for i in 1..1000 {
            alu.run(&vec![i]);
            assert_eq!(alu.data[1], -i);
        }
    }

    #[test]
    fn test_b() {
        let mut alu = ALU::from_file("ex/day24b.txt").unwrap();
        for i in 1..1000 {
            for j in -1..=1 {
                let second = i * 3 + j;
                alu.run(&vec![i, second]);
                assert_eq!(alu.data[3], if second == i * 3 {1} else {0});
            }
        }
    }

    #[test]
    fn test_c() {
        let mut alu = ALU::from_file("ex/day24c.txt").unwrap();
        for i in 1..1000 {
            alu.run(&vec![i]);
            let target = i % 16;
            let mut output = 0;
            for j in 0..alu.data.len() {
                output *= 2;
                output += alu.data[j];
            }
            assert_eq!(target, output);
            alu.reset();
        }
    }

    fn expand(i: isize) -> Vec<isize> {
        (0..MODEL_NUM_LEN).map(|d| i + d as isize).collect()
    }

    #[test]
    fn test_puzzle_input() {
        let mut alu = ALU::from_file("in/day24.txt").unwrap();
        for i in 1..=9 {
            let input = expand(i);
            alu.run(&input);
            println!("{}, {:?}", alu.data[3], input);
            alu.reset();
        }
    }
}