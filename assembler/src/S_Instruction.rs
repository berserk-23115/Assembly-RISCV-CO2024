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
fn format12(decimal: i32) -> String {
    format!("{:012b}", decimal)
}

fn s_type(input_str: &str) // taking input string as parameter , output file header
{
    // hashmap for register along with its binary encoding(string)
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

    let instruction: Vec<&str> = input_str.split(" ").collect();

    let operands: Vec<&str> = instruction[1].split(",").collect(); //SEPERATES {Return Address Register(ABI)} AND {Source Register1(ABI)}{,}{Immediate[11:0}

    let register_bin: &str = registers[operands[0]]; // recieved the Return Address Register binary code

    let imm_and_source_reg: Vec<&str> = operands[1].split("(").collect(); // seperated Immediate && Source Register

    let source_reg: &str = imm_and_source_reg[1];
    let source_reg: &str = &source_reg.replace(")", "");

    let imm: i32 = imm_and_source_reg[0].parse().unwrap(); // recieved the imm value in i32

    //converting decimal to binary(2s complement)
    let mut immediate_bin: String;
    if imm < 0 {
        immediate_bin = format12(imm.abs());
        immediate_bin = twos_complement(&immediate_bin);
    } else {
        immediate_bin = format12(imm);
    }

    println!(
        "{} {} {} {} {} {}",
        &immediate_bin[0..7],
        register_bin,
        registers[&source_reg],
        "010",
        &immediate_bin[7..11],
        "0100011"
    );
}
fn main() {
    let st: &str = "sw ra,32(sp)";
    s_type(st);
}
