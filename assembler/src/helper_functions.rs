pub fn remove_commas(input: &str) -> &str {
    input.trim_end_matches(',')
}

pub fn decimal_to_12bit_binary(decimal: u16) -> String {
    format!("{:012b}", decimal)
}

pub fn is_syntax_error(lines: Vec<Vec<&str>>) -> bool {
    let instructions = vec![
        "add", "sub", "sll", "slt", "sltu", "xor", "srl", "or", "and", "lw", "addi", "sltiu",
        "jalr", "sw", "beq", "bne", "blt", "bge", "bltu", "bgeu", "lui", "auipc", "jal", "mul",
        "rst", "halt", "rvrs",
    ];

    let mut syntax_errors = Vec::new();
    let mut ptr = 1;

    for line in lines {
        let length = line.len();
        print!("{}", length);
        if !instructions.contains(&line[0]) && length != 1 {
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
