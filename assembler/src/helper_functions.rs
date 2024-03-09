use crate::instructions;

pub fn remove_commas(input: &str) -> &str {
    input.trim_end_matches(',')
}

pub fn decimal_to_12bit_binary(decimal: u16) -> String {
    format!("{:012b}", decimal)
}

pub fn format19(value: i32) -> String {
    format!("{:019b}", value)
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
        vec!["mul", "rst", "halt", "rvrs"],
    ];
    let exception_instructions: Vec<&str> = vec!["lw", "sw"];

    let registers = vec![
        "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "fp", "s1", "a0", "a1", "a2", "a3",
        "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3",
        "t4", "t5", "t6", "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11",
        "x12", "x13", "x14", "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24",
        "x25", "x26", "x27", "x28", "x29", "x30", "x31",
    ];

    let mut syntax_errors = Vec::new();
    let mut ptr = 1;

    for line in lines {
        let length = line.len();
        print!("{}", length);
        if (instructions[0].contains(&line[0])
            && registers.contains(&line[1])
            && registers.contains(&line[2])
            && registers.contains(&line[3]))
            || (instructions[1].contains(&line[0])
                && registers.contains(&line[1])
                && registers.contains(&line[2]))
        {
            syntax_errors.push(ptr);
        }
        ptr += 1;
    }

    if syntax_errors.is_empty() {
        return false;
    } else {
        for line in syntax_errors {
            print!("Syntax Error in line no: {}\n", line);
        }
        return true;
    }
}
