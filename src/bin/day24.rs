use std::collections::VecDeque;
use std::io;
use std::num::ParseIntError;
use std::str::FromStr;
use advent_code_lib::{advent_main, all_lines, make_inner_io_error, make_io_error};
use bare_metal_modulo::{MNum, ModNumC};
use itertools::Itertools;

const VAR_NAMES: [char; 4] = ['w', 'x', 'y', 'z'];
const NUM_VARS: usize = VAR_NAMES.len();

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
enum ALUInstruction {
    Add(ModNumC<usize,NUM_VARS>, ALUArg2), Mul(ModNumC<usize,NUM_VARS>, ALUArg2),
    Div(ModNumC<usize,NUM_VARS>, ALUArg2), Mod(ModNumC<usize,NUM_VARS>, ALUArg2),
    Eql(ModNumC<usize,NUM_VARS>, ALUArg2), Inp(ModNumC<usize,NUM_VARS>)
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
            match instruction {
                ALUInstruction::Add(v, v2) => {
                    self.data[v.a()] = self.data[v.a()] + v2.value(&self.data);}
                ALUInstruction::Mul(v, v2) => {
                    self.data[v.a()] = self.data[v.a()] * v2.value(&self.data);}
                ALUInstruction::Div(v, v2) => {
                    self.data[v.a()] = self.data[v.a()] / v2.value(&self.data);}
                ALUInstruction::Mod(v, v2) => {
                    self.data[v.a()] = self.data[v.a()] % v2.value(&self.data);}
                ALUInstruction::Eql(v, v2) => {
                    self.data[v.a()] = if self.data[v.a()] == v2.value(&self.data) {1} else {0}}
                ALUInstruction::Inp(v) => {
                    self.data[v.a()] = input_queue.pop_front().unwrap();
                }
            }
        }
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
        let param = var_from(parts[1])?;
        if parts[0] == "inp" {
            Ok(ALUInstruction::Inp(param))
        } else {
            let arg2 = parts[2].parse::<ALUArg2>()?;
            match parts[0] {
                "add" => Ok(ALUInstruction::Add(param, arg2)),
                "mul" => Ok(ALUInstruction::Mul(param, arg2)),
                "div" => Ok(ALUInstruction::Div(param, arg2)),
                "mod" => Ok(ALUInstruction::Mod(param, arg2)),
                "eql" => Ok(ALUInstruction::Eql(param, arg2)),
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
        alu.run(&vec![24]);
        assert_eq!(alu.data[1], -24);
    }
}