use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn twos_complement(input: &str) -> String {
    let mut s: String = String::new();
    let mut cnt = 0;

    for ch in input.chars().rev() {
        if cnt < 1 {
            s.push(ch);
        } else {
            if ch == '0' {
                s.push('1');
            }
            if ch == '1' {
                s.push('0');
            }
        }

        if cnt < 1 && ch == '1' {
            cnt += 1;
        }
    }

    s.chars().rev().collect()
}
fn decimal_to_binary(decimal: u32) -> String {
    format!("{:012b}", decimal)
}

fn main() {
    let hash_map: HashMap<&str, Vec<&str>> = [
        ("addi", vec!["0010011", "000"]),
        ("lw", vec!["0000011", "010"]),
        ("lb", vec!["0000011", "010"]),
        ("lh", vec!["0000011", "010"]),
        ("ld", vec!["0000011", "010"]),
        ("sltiu", vec!["0010011", "011"]),
        ("jalr", vec!["1100111", "000"]),
    ]
    .iter()
    .cloned()
    .collect();
    let hash_map2: HashMap<&str, &str> = [
        ("x0", "00000"),
        ("x1", "00001"),
        ("x2", "00010"),
        ("x3", "00011"),
        ("x4", "00100"),
        ("x5", "00101"),
        ("x6", "00110"),
        ("x7", "00111"),
        ("x8", "01000"),
        ("x9", "01001"),
        ("x10", "01010"),
        ("x11", "01011"),
        ("x12", "01100"),
        ("x13", "01101"),
        ("x14", "01110"),
        ("x15", "01111"),
        ("x16", "10000"),
        ("x17", "10001"),
        ("x18", "10010"),
        ("x19", "10011"),
        ("x20", "10100"),
        ("x21", "10101"),
        ("x22", "10110"),
        ("x23", "10111"),
        ("x24", "11000"),
        ("x25", "11001"),
        ("x26", "11010"),
        ("x27", "11011"),
        ("x28", "11100"),
        ("x29", "11101"),
        ("x30", "11110"),
        ("x31", "11111"),
        // ABI REPRESENTATION OF REGISTERS
        ("zero", "00000"),
        ("ra", "00001"),
        ("sp", "00010"),
        ("gp", "00011"),
        ("tp", "00100"),
        ("t0", "00101"),
        ("t1", "00110"),
        ("t2", "00111"),
        ("s0", "01000"),
        ("s1", "01001"),
        ("a0", "01010"),
        ("a1", "01011"),
        ("a2", "01100"),
        ("a3", "01101"),
        ("a4", "01110"),
        ("a5", "01111"),
        ("a6", "10000"),
        ("a7", "10001"),
        ("s2", "10010"),
        ("s3", "10011"),
        ("s4", "10100"),
        ("s5", "10101"),
        ("s6", "10110"),
        ("s7", "10111"),
        ("s8", "11000"),
        ("s9", "11001"),
        ("s10", "11010"),
        ("s11", "11011"),
        ("t3", "11100"),
        ("t4", "11101"),
        ("t5", "11110"),
        ("t6", "11111"),
        ("fp", "01000"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut file = File::open("text4.txt").expect("can't open the file");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).expect("oops cant ...");

    for line in contents.lines() {
        println!("{line}");
        let opcode: Vec<&str> = line.split(" ").collect();
        let instruction: &str = opcode[0];
        let operands: Vec<&str> = opcode[1].split(",").collect();
        let instruction_bin: &Vec<&str> = &hash_map[instruction];
        let register_bin: &str = &hash_map2[operands[0]];
        let register_bin1: &str = &hash_map2[operands[1]];
        if let r = hash_map2[operands[2]].parse().unwrap() >= 0 {
            let register_bin2: &str = &decimal_to_binary(hash_map2[operands[2]].parse().unwrap());
        } else {
            let d: u32 = hash_map2[operands[2]].parse().unwrap().abs();
            let register = decimal_to_binary(d);
            let register_bin2: &str = &twos_complement(&register);
        }
        println!(
            "{}{}{}{}{}{}",
            instruction_bin[2],
            register_bin2,
            register_bin1,
            instruction_bin[1],
            register_bin,
            instruction_bin[0]
        );
    }
}
