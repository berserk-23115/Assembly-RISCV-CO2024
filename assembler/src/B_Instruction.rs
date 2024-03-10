// modules
mod helper_functions;
use helper_functions::*;
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
        let imm: i32 = my_string.parse().unwrap();

        let mut immediate_bin: String;
        if imm < 0 {
            immediate_bin = format12(imm.abs());
            immediate_bin = twos_complement(&immediate_bin);
        } else {
            immediate_bin = format12(imm);
        }

        s.push_str(&immediate_bin[0..7]);
        if let Some(value) = hash_map2.get(my_array[2]) {
            s.push_str(value);
        }
        if let Some(value) = hash_map2.get(my_array[1]) {
            s.push_str(value);
        }
        if let Some(value) = hash_map.get(my_array[0]) {
            s.push_str(value[1]);
        }
        s.push_str(&immediate_bin[7..]);
        if let Some(value) = hash_map.get(my_array[0]) {
            s.push_str(value[0]);
        }

        print!("{s}");
    }
}
