// modules
mod helper_functions;
use helper_functionj::twos_complement;
use helper_functions::decimal_to_12bit_binary;
use helper_functions::remove_commas;
mod instructions;

// header files
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // opcode func3
    let hash_map: HashMap<&str, Vec<&str>> = [
        ("beq", vec!["1100011", "000"]),
        ("bne", vec!["1100011", "001"]),
        ("blt", vec!["1100011", "100"]),
        ("bge", vec!["1100011", "101"]),
        ("bltu", vec!["1100011", "110"]),
        ("bgeu", vec!["1100011", "111"]),
    ]
    .iter()
    .cloned()
    .collect();

    // registers
    let hash_map2: HashMap<&str, &str> = [
        ("R0", "00000"),
        ("R1", "00001"),
        ("R2", "00010"),
        ("R3", "00011"),
        ("R4", "00100"),
        ("R5", "00101"),
        ("R6", "00110"),
        ("R7", "00111"),
        ("R8", "01000"),
        ("R9", "01001"),
        ("R10", "01010"),
        ("R11", "01011"),
        ("R12", "01100"),
        ("R13", "01101"),
        ("R14", "01110"),
        ("R15", "01111"),
        ("R16", "10000"),
        ("R17", "10001"),
        ("R18", "10010"),
        ("R19", "10011"),
        ("R20", "10100"),
        ("R21", "10101"),
        ("R22", "10110"),
        ("R23", "10111"),
        ("R24", "11000"),
        ("R25", "11001"),
        ("R26", "11010"),
        ("R27", "11011"),
        ("R28", "11100"),
        ("R29", "11101"),
        ("R30", "11110"),
        ("R31", "11111"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut file = File::open("input.txt").expect("can't open the file");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).expect("oops cant ...");

    let mut arr: Vec<Vec<&str>> = vec![];

    for line in contents.lines() {
        let mut my_array: Vec<&str> = vec![];
        let x = line.split_whitespace();
        for y in x {
            let w = remove_commas(y);
            my_array.push(&w);
        }
        arr.push(my_array.clone());
        let mut s: String = String::new();

        let my_string = my_array[3];
        let my_int: i32 = my_string.parse().unwrap();
        print!("{my_int}\n");

        let imm_binary = decimal_to_12bit_binary(my_int as u16);

        print!("{imm_binary}");
        // imm[5:12]
        s.push_str(&imm_binary[0..7]);
        // src reg2
        if let Some(value) = hash_map2.get(my_array[2]) {
            s.push_str(value);
        }
        // src reg1
        if let Some(value) = hash_map2.get(my_array[1]) {
            s.push_str(value);
        }
        // func 3
        if let Some(value) = hash_map.get(my_array[0]) {
            s.push_str(value[1]);
        }
        // imm[4:0]
        s.push_str(&imm_binary[7..]);
        // opcode
        if let Some(value) = hash_map.get(my_array[0]) {
            s.push_str(value[0]);
        }
    }
}
