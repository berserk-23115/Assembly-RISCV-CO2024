use std::collections::HashMap;
// use std::fs::File;
// use std::io::prelude::*;

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

fn format11(value: i32) -> String {
    format!("{:011b}", value) // format!("{:012b}", value)
}

fn i_type(input_str: &str) {
    let instructions: HashMap<&str, &str> = [
        ("addi", "0010011"),
        ("lw", "0000011"),
        // ("lb", "0000011"),
        // ("lh", "0000011"),
        // ("ld", "0000011"),
        ("sltiu", "0010011"),
        ("jalr", "1100111"),
    ]
    .iter()
    .cloned()
    .collect();
    let func_code: HashMap<&str, &str> = [
        ("addi", "000"),
        ("lw", "010"),
        // ("lb", "010"),
        // ("lh", "010"),
        // ("ld", "010"),
        ("sltiu", "011"),
        ("jalr", "000"),
    ]
    .iter()
    .cloned()
    .collect();

    let registers: HashMap<&str, &str> = [
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

    let mut string_parsed: Vec<&str> = vec![];
    string_parsed = input_str.split(" ").collect();
    let mut opcode = string_parsed[0];

    let mut operands: Vec<&str> = vec![];
    operands = string_parsed[1].split(",").collect();
    let source_reg: &str = registers[operands[1]];
    let des_reg: &str = registers[operands[0]];
    let immediate: i32 = operands[2].parse().unwrap();
    let mut immediate_bin: String;
    if immediate >= 0 {
        immediate_bin = format11(immediate);
    } else {
        immediate_bin = format11(immediate.abs());
        immediate_bin = twos_complement(&immediate_bin);
    }
    println!(
        "{} {} {} {} {}",
        immediate_bin, source_reg, func_code[opcode], des_reg, instructions[opcode]
    );
}

fn main() {
    let str1: &str = "jalr ra,a5,-7";
    i_type(str1);
}

