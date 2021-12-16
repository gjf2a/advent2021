use std::io;
use std::str::{Chars, FromStr};
use advent_code_lib::{advent_main, make_inner_io_error, make_io_error};
use bits::BitArray;
use itertools::Itertools;
use num::{BigUint, Zero};

const VERSION_LENGTH: usize = 3;
const OP_TYPE_LENGTH: usize = 3;
const LITERAL_GROUP_LENGTH: usize = 4;
const SUB_PACKETS_LENGTH: usize = 15;
const SUB_PACKETS_COUNT: usize = 11;

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        Ok(())
    })
}

#[derive(Copy, Clone, Eq, Debug, PartialEq)]
enum OpCode {
    Literal, Zero, One, Two, Three, Five, Six, Seven
}

#[derive(Clone, Eq, Debug, PartialEq)]
enum Packet {
    Literal(BigUint, BigUint),
    Operator(BigUint, OpCode, Vec<Packet>)
}

impl Packet {
    fn version_sum(&self) -> BigUint {
        match self {
            Packet::Literal(version, _) => version.clone(),
            Packet::Operator(version, _, children) => {
                version + &children.iter().map(|child| child.version_sum()).sum::<BigUint>()
            }
        }
    }
}

impl FromStr for OpCode {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
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
        let (packet, _size) = parse_next_packet(&mut hex2binary(s)?.chars())?;
        Ok(packet)
    }
}

fn parse_next_packet(iter: &mut Chars) -> io::Result<(Packet, usize)> {
    let version = bits2bigint(iter, VERSION_LENGTH)?;
    let (packet, count) = match bits2string(iter, OP_TYPE_LENGTH).parse::<OpCode>()? {
        OpCode::Literal => {
            let (literal, count) = parse_literal(iter)?;
            (Packet::Literal(version, literal), count)
        }
        operator => {
            let (sub_packets, count) = parse_operator(iter)?;
            (Packet::Operator(version, operator, sub_packets), count)
        }
    };
    Ok((packet, count + VERSION_LENGTH + OP_TYPE_LENGTH))
}

fn parse_literal(iter: &mut Chars) -> io::Result<(BigUint, usize)> {
    let mut actual_bits = String::new();
    let mut bits_used = 0;
    loop {
        let header = iter.next().ok_or(make_inner_io_error("Out of chars!"))?;
        bits_used += 1;
        actual_bits.push_str(bits2string(iter, LITERAL_GROUP_LENGTH).as_str());
        bits_used += LITERAL_GROUP_LENGTH;
        if header == '0' {break;}
    }
    Ok((BigUint::from(&actual_bits.parse::<BitArray>()?), bits_used))
}

fn parse_operator(iter: &mut Chars) -> io::Result<(Vec<Packet>, usize)> {
    let mut packets = Vec::new();
    let mut bits_used = 1;
    match iter.next().ok_or(make_inner_io_error("No length type"))? {
        '0' => {
            let mut length = bits2bigint(iter, SUB_PACKETS_LENGTH)?;
            bits_used += SUB_PACKETS_LENGTH;
            while length > BigUint::zero() {
                let (packet, used) = parse_next_packet(iter)?;
                length -= BigUint::from(used);
                bits_used += used;
                packets.push(packet);
            }
        },
        '1' => {
            let count = bits2bigint(iter, SUB_PACKETS_COUNT)?;
            bits_used += SUB_PACKETS_COUNT;
            for _ in num::range(BigUint::zero(), count) {
                let (packet, used) = parse_next_packet(iter)?;
                bits_used += used;
                packets.push(packet);
            }
        },
        other => return make_io_error(format!("Unrecognized char: {}", other).as_str())
    }
    Ok((packets, bits_used))
}

fn bits2string(iter: &mut Chars, bits_to_take: usize) -> String {
    iter.take(bits_to_take).by_ref().collect()
}

fn bits2bits(iter: &mut Chars, bits_to_take: usize) -> io::Result<BitArray> {
    bits2string(iter, bits_to_take).parse()
}

fn bits2bigint(iter: &mut Chars, bits_to_take: usize) -> io::Result<BigUint> {
    Ok(BigUint::from(&bits2bits(iter, bits_to_take)?))
}

fn hex2binary(hex: &str) -> io::Result<String> {
    Ok(hex.chars()
        .map(|c| char_matcher(c))
        .fold_ok(String::new(), |mut s, c| {s.push_str(c.as_str()); s})?)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        for (hex, version_sum) in [
            ("D2FE28", 6),
            ("38006F45291200", 1 + 6 + 2),
            ("EE00D40C823060", 7 + 2 + 4 + 1),
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31)
        ] {
            println!("Hex: {} (sum: {})", hex, version_sum);
            let packet: Packet = hex.parse().unwrap();
            println!("{:?}", packet);
            assert_eq!(packet.version_sum(), BigUint::from(version_sum as usize));
        }
    }
}