use std::io;
use std::str::{Chars, FromStr};
use advent_code_lib::{advent_main, all_lines, make_inner_io_error, make_io_error};
use bits::BitArray;
use itertools::Itertools;
use num::{BigUint, One, Zero};

const VERSION_LENGTH: usize = 3;
const OP_TYPE_LENGTH: usize = 3;
const LITERAL_GROUP_LENGTH: usize = 4;
const SUB_PACKETS_LENGTH: usize = 15;
const SUB_PACKETS_COUNT: usize = 11;

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| {
        let packet: Packet = all_lines(args[1].as_str())?.next().unwrap().parse()?;
        println!("Part 1: {}", packet.version_sum());
        println!("Part 2: {}", packet.calculate());
        Ok(())
    })
}

#[derive(Copy, Clone, Eq, Debug, PartialEq)]
enum AllOp {
    Sum, Product, Minimum, Maximum
}

#[derive(Copy, Clone, Eq, Debug, PartialEq)]
enum TwoOp {
    Greater, Less, Equal
}

#[derive(Clone, Eq, Debug, PartialEq)]
enum Packet {
    Literal(BigUint, BigUint),
    AllOperator(BigUint, AllOp, Vec<Packet>),
    TwoOperator(BigUint, TwoOp, Box<Packet>, Box<Packet>)
}

impl Packet {
    fn version_sum(&self) -> BigUint {
        match self {
            Packet::Literal(version, _) => version.clone(),
            Packet::AllOperator(version, _, children) => {
                version + &children.iter().map(|child| child.version_sum()).sum::<BigUint>()
            }
            Packet::TwoOperator(version, _, one, two) => {
                version + one.version_sum() + two.version_sum()
            }
        }
    }

    fn calculate(&self) -> BigUint {
        match self {
            Packet::Literal(_, value) => value.clone(),
            Packet::AllOperator(_, opcode, sub_packets) => opcode.calculate(sub_packets),
            Packet::TwoOperator(_, opcode, sub1, sub2) => opcode.calculate(sub1, sub2)
        }
    }
}

impl AllOp {
    fn calculate(&self, sub_packets: &Vec<Packet>) -> BigUint {
        let subs = sub_packets.iter().map(|p| p.calculate());
        match self {
            AllOp::Sum => subs.sum(),
            AllOp::Product => subs.product(),
            AllOp::Minimum => subs.min().unwrap(),
            AllOp::Maximum => subs.max().unwrap()
        }
    }
}

impl TwoOp {
    fn calculate(&self, sub1: &Packet, sub2: &Packet) -> BigUint {
        let calc1 = sub1.calculate();
        let calc2 = sub2.calculate();
        if match self {
            TwoOp::Greater => calc1 > calc2,
            TwoOp::Less => calc1 < calc2,
            TwoOp::Equal => calc1 == calc2
        } {BigUint::one()} else {BigUint::zero()}
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
    let op_type = bits2string(iter, OP_TYPE_LENGTH);
    let (packet, count) = match op_type.chars().next().unwrap() {
        '0' => {
            let (sub_packets, count) = parse_operator(iter)?;
            let op = match op_type.as_str() {
                "000" => AllOp::Sum,
                "001" => AllOp::Product,
                "010" => AllOp::Minimum,
                "011" => AllOp::Maximum,
                other => return make_io_error(format!("Unrecognized OpCode: {}", other).as_str())
            };
            (Packet::AllOperator(version, op, sub_packets), count)
        }
        _ => {
            if op_type.as_str() == "100" {
                let (literal, count) = parse_literal(iter)?;
                (Packet::Literal(version, literal), count)
            } else {
                let (sub_packets, count) = parse_operator(iter)?;
                let op = match op_type.as_str() {
                    "101" => TwoOp::Greater,
                    "110" => TwoOp::Less,
                    "111" => TwoOp::Equal,
                    other => return make_io_error(format!("Unrecognized OpCode: {}", other).as_str())
                };
                (Packet::TwoOperator(version, op, Box::new(sub_packets[0].clone()), Box::new(sub_packets[1].clone())), count)
            }
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

    #[test]
    fn test_part_2() {
        for (hex, value) in [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ] {
            println!("Hex: {} (sum: {})", hex, value);
            let packet: Packet = hex.parse().unwrap();
            println!("{:?}", packet);
            assert_eq!(packet.calculate(), BigUint::from(value as usize));

        }
    }
}