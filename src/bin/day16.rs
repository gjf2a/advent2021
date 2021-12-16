use std::io;
use std::str::{Chars, FromStr};
use advent_code_lib::{advent_main, make_io_error};
use bits::BitArray;
use num::BigUint;

const VERSION_LENGTH: usize = 3;
const OP_TYPE_LENGTH: usize = 3;
const LITERAL_GROUP_LENGTH: usize = 4;
const SUB_PACKETS_LENGTH: usize = 15;

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        Ok(())
    })
}

#[derive(Copy, Clone, Eq)]
enum OpCode {
    Literal, Zero, One, Two, Three, Five, Six, Seven
}

#[derive(Clone, Eq)]
enum Packet {
    Literal(BigUint, BigUint),
    Operator(BigUint, OpCode, Vec<Packet>)
}

impl FromStr for OpCode {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match bits {
            "000" => OpCode::Zero,
            "001" => OpCode::One,
            "010" => OpCode::Two,
            "011" => OpCode::Three,
            "100" => OpCode::Literal,
            "101" => OpCode::Five,
            "110" => OpCode::Six,
            "111" => OpCode::Seven,
            other => return make_io_error(format!("Unrecognized OpCode: {}", other).as_str())
        })
    }
}

impl FromStr for Packet {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_next_packet(&mut hex2binary(s)?.chars())
    }
}

fn parse_next_packet(iter: &mut Chars) -> io::Result<Packet> {
    let version = bits2bigint(iter, VERSION_LENGTH)?;
    Ok(match bits2string(iter, OP_TYPE_LENGTH).parse::<OpCode>()? {
        OpCode::Literal => Packet::Literal(version, parse_literal(iter)?),
        operator => Packet::Operator(version, operator, parse_operator(iter)?)
    })
}

fn parse_literal(iter: &mut Chars) -> io::Result<BigUint> {
    let mut actual_bits = String::new();
    loop {
        let header = iter.next().ok_or_else(make_io_error("Out of chars!"))?;
        actual_bits.push_str(bits2string(iter, LITERAL_GROUP_LENGTH)?);
        if header == '0' {break;}
    }
    Ok(BigUint::from(actual_bits.parse::<BitArray>()?))
}

fn parse_operator(iter: &mut Chars) -> io::Result<Vec<Packet>> {
    let mut packets = Vec::new();
    match iter.next().ok_or_else(make_io_error("No length type"))? {
        '0' => {
            let length = bits2bigint(iter, SUB_PACKETS_LENGTH)?;

        },
        '1' => {

        },
        other => return make_io_error(format!("Unrecognized char: {}", other).as_str())
    }
    Ok(packets)
}

fn bits2string(iter: &mut Chars, bits_to_take: usize) -> String {
    iter.take(bits_to_take).by_ref().collect()
}

fn bits2bits(iter: &mut Chars, bits_to_take: usize) -> io::Result<BitArray> {
    bits2string(iter, bits_to_take).parse()
}

fn bits2bigint(iter: &mut Chars, bits_to_take: usize) -> io::Result<BigUint> {
    Ok(BigUint::from(bits2bits(iter, bits_to_take)?))
}

fn hex2binary(hex: &str) -> io::Result<String> {
    Ok(hex.chars().map(|c| char_matcher(c)?).collect())
}

fn char_matcher(c: char) -> io::Result<String> {
    Ok(match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        other => { return make_io_error(format!("Unrecognized hex digit: {}", other).as_str()); }
    }.to_string())
}