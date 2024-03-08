// mod helper_functions;
// use helper_functions::decimal_to_12bit_binary;
// use helper_functions::remove_commas;

use std::collections::HashMap;

fn decimal_to_12bit_binary(num: u16) -> String {
    format!("{:012b}", num)
}

fn j_type(input_str: &str) {
    let instructions: HashMap<&str,&str> = [("jal","0010111")].iter().cloned().collect();
    let registers: HashMap<&str, &str> = [("x0", "00000"), /*...*/ ("fp", "01000")].iter().cloned().collect();

    let opcode: Vec<&str> = input_str.split(" ").collect();
    let opstr: &str = opcode[0];
    let operands: Vec<&str> = opcode[1].split(",").collect();

    let immediate_val: &str = operands[1];
    let imm_int: i32 = immediate_val.parse().unwrap();
    let imm_bin: String = decimal_to_12bit_binary(imm_int.abs() as u16);

    let mut register_bin: &str = "";
    for i in registers.keys() {
        if operands[0] == *i {
            register_bin = registers[i];
        }
    }

    let final_str = format!("{}{}{}", instructions[opstr], register_bin, imm_bin);
    println!("{}", final_str);
}

fn main() {
    let str1: &str = "jal ra,-1024";
    j_type(str1);
}