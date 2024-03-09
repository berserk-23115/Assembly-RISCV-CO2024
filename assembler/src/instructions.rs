use crate::helper_functions::decimal_to_12bit_binary;
use crate::helper_functions::format19;

use std::collections::HashMap;
use std::io::prelude::*;

pub fn rtype(
    hash_map: &HashMap<&str, Vec<&str>>,
    hash_map2: &HashMap<&str, &str>,
    my_array: &Vec<&str>,
) -> String {
    let mut s: String = String::new();

    let instruction_bin: &Vec<&str> = &hash_map[my_array[0]];
    let register_bin: &str = &hash_map2[my_array[1]];
    let register_bin1: &str = &hash_map2[my_array[2]];
    let register_bin2: &str = &hash_map2[my_array[3]];

    s.push_str(instruction_bin[2]);
    s.push_str(register_bin2);
    s.push_str(register_bin1);
    s.push_str(instruction_bin[1]);
    s.push_str(register_bin);
    s.push_str(instruction_bin[0]);

    s
}

pub fn btype(
    hash_map: HashMap<&str, Vec<&str>>,
    hash_map2: HashMap<&str, &str>,
    my_array: Vec<&str>,
) -> String {
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
    s
}

pub fn utype(
    hash_map: &HashMap<&str, &str>,
    hash_map2: &HashMap<&str, &str>,
    my_array: &Vec<&str>,
) -> String {
    let mut s: String = String::new();

    let instruction_bin: &str = &hash_map[my_array[0]];
    let dest_register_bin: &str = &hash_map2[my_array[1]];
    let immediate_bin: String = format19(my_array[2].parse().unwrap());

    s.push_str(&immediate_bin);
    s.push_str(dest_register_bin);
    s.push_str(instruction_bin);

    s
}
