

// header files
use std::collections::HashMap;

fn format19(value: i32) -> String {
    format!("{:031b}", value) // Ensure 31 bits are used for representation
}

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

fn u_type(input_str: &str) {
    // taking input string as parameter , output file header
    // hashmap for instructions in the instruction category
    let instructions: HashMap<&str, &str> = [
        //instrtuction mneumonic, opcode, function code = NULL ;
        ("lui", "0110111"),
        ("auipc", "0010111"),
    ]
    .iter()
    .cloned()
    .collect();
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
    let opcode: Vec<&str> = input_str.split(" ").collect();
    let instruction: &str = opcode[0];
    let operands: Vec<&str> = opcode[1].split(",").collect();
    let instruction_bin: &str = instructions[instruction];
    let dest_register_bin: &str = registers[operands[0]];
    // let immediate_bin: String = format19(operands[1].parse().unwrap());
    let immediate: i32 = operands[1].parse().unwrap();
    let mut immediate_bin: String;
    if immediate >= 0 {
        immediate_bin = format19(immediate);
    } else {
        immediate_bin = format19(immediate.abs());
        immediate_bin = twos_complement(&immediate_bin);
    }
    if immediate_bin.len() < 31 {
        immediate_bin = format!("{:0>31}", immediate_bin);
    }
    println!(
        "{} {} {}",
        &immediate_bin[0..20], // Only print the first 20 bits
        dest_register_bin,
        instruction_bin
    );
}

fn main() {
    let str1: &str = "auipc s2,-30";
    u_type(str1);
}
