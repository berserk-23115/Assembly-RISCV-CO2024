mod helper_functions;
use helper_functions::decimal_to_12bit_binary;
// use helper_functions::remove_commas;

// header files
use std::collections::HashMap;
// use std::fs::File;
// use std::io::prelude::*;

/*

IC DES REG IMM
 */

fn U_type(input_str: &str) {
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
    let mut opcode: Vec<&str> = vec![];
    opcode = input_str.split(" ").collect(); // opcode isolation from string
    let instruction: &str = opcode[0];
    let opstr: &str = opcode[1];
    let mut operands: Vec<&str> = vec![]; // operand isolation from string
    operands = opstr.split(",").collect(); // operand split
    let final_str: String;
    let instruction_bin: &str;
    let register_bin: &str;

    // IMMEDIATE VALUE CONVERSION
    let immediate_val: &str;
    let imm_int: u16 = immediate_val.parse().unwrap();
    let imm_bin: String = decimal_to_12bit_binary(imm_int);

    // REGISTER BINARY CONVERSION
    for i in registers.keys() {
        if &operands[0] == i {
            register_bin = registers[operands[0]];
        }
    }
    for j in instructions.keys() {
        if assert_eq!(j, instruction){
            instruction_bin = instructions[instruction];
        }
    }
    final_str = instruction_bin.to_owned() + register_bin + &imm_bin;
    println!("{}", final_str);
}
fn main() {

}
