mod helper_functions;
use helper_functions::decimal_to_12bit_binary;
use std::collections::HashMap;
use std::io::prelude::*;
use lazy_static::lazy_static;
// opcode func3
lazy_static! {
    static ref hash_map: HashMap<&'static str, Vec<&str> = {
    let hash_map = [
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
    }
    static ref hash_map2: HashMap<&'static str, Vec<&str> = {
    let hash_map2 = [
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
}
    }

pub fn Rtype(my_array: Vec<&str>) -> String {
    let mut s: String = String::new();

    if let Some(value) = hash_map.get(my_array[0]) {
        s.push_str(value[2]);
    }
    if let Some(value) = hash_map2.get(my_array[3]) {
        s.push_str(value);
    }
    if let Some(value) = hash_map2.get(my_array[2]) {
        s.push_str(value);
    }
    if let Some(value) = hash_map.get(my_array[0]) {
        s.push_str(value[1]);
    }
    if let Some(value) = hash_map2.get(my_array[1]) {
        s.push_str(value);
    }
    if let Some(value) = hash_map.get(my_array[0]) {
        s.push_str(value[0]);
    }
    return s;
}

pub fn Itype(my_array: Vec<&str>) -> String {
    let mut s: String = String::new();

    let my_int: u16 = my_array[3].parse().unwrap();
    let binary_number = decimal_to_12bit_binary(my_int);
    let v: String = binary_number.to_string();

    s.push_str(&v);
    if let Some(value) = hash_map2.get(my_array[2]) {
        s.push_str(value);
    }
    if let Some(value) = hash_map.get(my_array[0]) {
        s.push_str(value[1]);
    }
    if let Some(value) = hash_map2.get(my_array[1]) {
        s.push_str(value);
    }
    if let Some(value) = hash_map.get(my_array[0]) {
        s.push_str(value[0]);
    }
    return s;
}

pub fn Btype(my_array: Vec<&str>) -> String {
    let mut s: String = String::new();

    let my_string = my_array[3];
    let my_int: u16 = my_string.parse().unwrap();

    let imm_binary = decimal_to_12bit_binary(my_int);

    s.push_str(&imm_binary[0..7]);
    if let Some(value) = hash_map2.get(my_array[2]) {
        s.push_str(value);
    }
    if let Some(value) = hash_map2.get(my_array[1]) {
        s.push_str(value);
    }
    if let Some(value) = hash_map.get(my_array[0]) {
        s.push_str(value[1]);
    }
    s.push_str(&imm_binary[7..]);
    if let Some(value) = hash_map.get(my_array[0]) {
        s.push_str(value[0]);
    }
    return s;
}
