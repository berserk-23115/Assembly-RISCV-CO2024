// modules
mod helper_functions;
use helper_functions::decimal_to_12bit_binary;
use helper_functions::remove_commas;
use helper_functions::syntax_checker;
mod instructions;
use instructions::Btype;

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
        arr.push(my_array);
        let s: String = Btype(my_array);
        println!("{s}");
    }
    syntax_checker(arr);
}
