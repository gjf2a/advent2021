use std::io;
use std::str::Chars;
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
        let line = all_lines(args[1].as_str())?.next().unwrap();
        let (version_sum, calculation) = version_and_calcluation(line.as_str())?;
        println!("Part 1: {}", version_sum);
        println!("Part 2: {}", calculation);
        Ok(())
    })
}

fn version_and_calcluation(line: &str) -> io::Result<(BigUint, BigUint)> {
    let binarized = hex2binary(line)?;
    let (version_sum, calculation, _count) = parse_next_packet(&mut binarized.chars())?;
    Ok((version_sum, calculation))
}

fn parse_next_packet(iter: &mut Chars) -> io::Result<(BigUint, BigUint, usize)> {
    let version = bits2bigint(iter, VERSION_LENGTH)?;
    let op_type = bits2string(iter, OP_TYPE_LENGTH);
    let (version_sum, calculation, count) = match op_type.chars().next().unwrap() {
        '0' => parse_zero_operator(op_type.as_str(), iter)?,
        _ => parse_one_operator(op_type.as_str(), iter)?
    };
    Ok((version + version_sum, calculation, count + VERSION_LENGTH + OP_TYPE_LENGTH))
}

fn parse_zero_operator(op_type: &str, iter: &mut Chars) -> io::Result<(BigUint, BigUint, usize)> {
    let (version_sum, sub_packets, count) = parse_sub_packets(iter)?;
    let calculation = match op_type {
        "000" => sub_packets.iter().sum::<BigUint>(),
        "001" => sub_packets.iter().product::<BigUint>(),
        "010" => sub_packets.iter().min().unwrap().clone(),
        "011" => sub_packets.iter().max().unwrap().clone(),
        other => return make_io_error(format!("Unrecognized OpCode: {}", other).as_str())
    };
    Ok((version_sum, calculation.clone(), count))
}

fn parse_one_operator(op_type: &str, iter: &mut Chars) -> io::Result<(BigUint, BigUint, usize)> {
    if op_type == "100" {
        let (literal, count) = parse_literal(iter)?;
        Ok((BigUint::zero(), literal, count))
    } else {
        let (version_sum, sub_packets, count) = parse_sub_packets(iter)?;
        let op = match op_type {
            "101" => sub_packets[0] > sub_packets[1],
            "110" => sub_packets[0] < sub_packets[1],
            "111" => sub_packets[0] == sub_packets[1],
            other => return make_io_error(format!("Unrecognized OpCode: {}", other).as_str())
        };
        let value = if op {BigUint::one()} else {BigUint::zero()};
        Ok((version_sum, value, count))
    }
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

fn parse_sub_packets(iter: &mut Chars) -> io::Result<(BigUint, Vec<BigUint>, usize)> {
    let mut packets = Vec::new();
    let mut version_sum = BigUint::zero();
    let mut bits_used = 1;
    match iter.next().ok_or(make_inner_io_error("No length type"))? {
        '0' => parse_sub_0(iter, &mut bits_used, &mut version_sum, &mut packets)?,
        '1' => parse_sub_1(iter, &mut bits_used, &mut version_sum, &mut packets)?,
        other => return make_io_error(format!("Unrecognized char: {}", other).as_str())
    }
    Ok((version_sum, packets, bits_used))
}

fn parse_sub_0(iter: &mut Chars, bits_used: &mut usize, version_sum: &mut BigUint, packets: &mut Vec<BigUint>) -> io::Result<()> {
    let mut length = bits2bigint(iter, SUB_PACKETS_LENGTH)?;
    *bits_used += SUB_PACKETS_LENGTH;
    while length > BigUint::zero() {
        let (version, packet, used) = parse_next_packet(iter)?;
        length -= BigUint::from(used);
        *bits_used += used;
        *version_sum += version;
        packets.push(packet);
    }
    Ok(())
}

fn parse_sub_1(iter: &mut Chars, bits_used: &mut usize, version_sum: &mut BigUint, packets: &mut Vec<BigUint>) -> io::Result<()> {
    let count = bits2bigint(iter, SUB_PACKETS_COUNT)?;
    *bits_used += SUB_PACKETS_COUNT;
    for _ in num::range(BigUint::zero(), count) {
        let (version, packet, used) = parse_next_packet(iter)?;
        *bits_used += used;
        *version_sum += version;
        packets.push(packet);
    }
    Ok(())
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
        for (hex, value) in [
            ("D2FE28", 6),
            ("38006F45291200", 1 + 6 + 2),
            ("EE00D40C823060", 7 + 2 + 4 + 1),
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31)
        ] {
            println!("Hex: {} (sum: {})", hex, value);
            let (version_sum, _calculation) = version_and_calcluation(hex).unwrap();
            assert_eq!(version_sum, BigUint::from(value as usize));
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
            let (_version_sum, calculation) = version_and_calcluation(hex).unwrap();
            assert_eq!(calculation, BigUint::from(value as usize));
        }
    }
}