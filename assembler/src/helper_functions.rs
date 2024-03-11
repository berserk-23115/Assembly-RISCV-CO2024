use crate::instructions;

use std::fs::File;
use std::io::{self, Write};

pub fn remove_commas(input: &str) -> &str {
    input.trim_end_matches(',')
}

pub fn format11(decimal: i32) -> String {
    format!("{:011b}", decimal)
}

pub fn format12(decimal: i32) -> String {
    format!("{:012b}", decimal)
}
pub fn format13(decimal: i32) -> String {
    format!("{:013b}", decimal)
}
pub fn format20(value: i32) -> String {
    format!("{:020b}", value)
}

pub fn format32(value: i32) -> String {
    format!("{:031b}", value)
}

pub fn check_range(value: &str, range: u32) -> bool {
    let value: f32 = value.parse().unwrap();
    if value >= -2.0_f32.powf(range as f32) && value < 2.0_f32.powf(range as f32) {
        return true;
    }
    false
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

pub fn is_syntax_error(lines: Vec<Vec<&str>>) -> bool {
    let instructions: Vec<Vec<&str>> = vec![
        vec![
            "add", "sub", "sll", "slt", "sltu", "xor", "srl", "or", "and",
        ],
        vec!["addi", "sltiu", "jalr"],
        vec!["beq", "bne", "blt", "bge", "bltu", "bgeu"],
        vec!["lui", "auipc"],
        vec!["jal"],
        vec!["lw", "sw"],
        vec!["mul", "rst", "halt", "rvrs"],
    ];

    let registers = vec![
        "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "fp", "s1", "a0", "a1", "a2", "a3",
        "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3",
        "t4", "t5", "t6", "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11",
        "x12", "x13", "x14", "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24",
        "x25", "x26", "x27", "x28", "x29", "x30", "x31",
    ];

    let mut syntax_errors = Vec::new();

    for (line_no, line) in lines.iter().enumerate() {
        let instruction = line[0];
        if instructions[0].contains(&instruction)
            && registers.contains(&line[1])
            && registers.contains(&line[2])
            && registers.contains(&line[3])
        {
            print!("");
        } else if instructions[1].contains(&instruction)
            && registers.contains(&line[1])
            && registers.contains(&line[2])
            && check_range(&line[3], 11)
        {
            print!("");
        } else if instructions[5].contains(&instruction) && registers.contains(&line[1]) {
            let imm_and_source_reg: Vec<&str> = line[2].split("(").collect();
            let source_reg: &str = imm_and_source_reg[1];
            let source_reg: &str = &source_reg.replace(")", "");

            if check_range(&imm_and_source_reg[0], 11) && registers.contains(&source_reg) {
                print!("");
            } else {
                syntax_errors.push(format!("Syntax Error in line {}: {:?}", line_no + 1, line));
            }
        } else if instructions[2].contains(&instruction)
            && registers.contains(&line[1])
            && registers.contains(&line[2])
        {
            print!("");
        } else if instructions[3].contains(&instruction)
            && registers.contains(&line[1])
            && check_range(&line[2], 31)
        {
            print!("");
        } else if instructions[4].contains(&instruction)
            && registers.contains(&line[1])
            && check_range(&line[2], 20)
        {
            print!("");
        } else {
            syntax_errors.push(format!("Syntax Error in line {}: {:?}", line_no + 1, line));
        }
    }

    if !syntax_errors.is_empty() {
        for error in &syntax_errors {
            println!("{}", error);
        }
        return true;
    }

    false
}

pub fn write_output(filename: &str, data: &Vec<String>) -> io::Result<()> {
    let mut file = File::create(filename)?;

    for item in data {
        writeln!(file, "{}", item)?;
    }

    Ok(())
}
