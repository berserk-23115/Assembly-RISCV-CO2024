use crate::helper_functions::*;

use std::collections::HashMap;

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

pub fn itype(
    hash_map: &HashMap<&str, Vec<&str>>,
    hash_map2: &HashMap<&str, &str>,
    my_array: &Vec<&str>,
) -> String {
    let mut s: String = String::new();

    if my_array[0] == "lw" {
        let register_bin: &str = hash_map2[my_array[1]]; // recieved the Return Address Register binary code
        let imm_and_source_reg: Vec<&str> = my_array[2].split("(").collect(); // seperated Immediate && Source Register
        let source_reg: &str = imm_and_source_reg[1];
        let source_reg: &str = &source_reg.replace(")", "");
        let imm: i32 = imm_and_source_reg[0].parse().unwrap(); // recieved the imm value in i32

        let mut immediate_bin: String;
        if imm < 0 {
            immediate_bin = format12(imm.abs());
            immediate_bin = twos_complement(&immediate_bin);
        } else {
            immediate_bin = format12(imm);
        }

        s.push_str(&immediate_bin);
        s.push_str(hash_map2[source_reg]);
        s.push_str("010");
        s.push_str(register_bin);
        s.push_str("0000011");

        return s;
    }

    let instruction_bin: &Vec<&str> = &hash_map[my_array[0]];
    let source_reg: &str = hash_map2[my_array[2]];
    let des_reg: &str = hash_map2[my_array[1]];
    let immediate: i32 = my_array[3].parse().unwrap();
    let mut immediate_bin: String;
    if immediate >= 0 {
        immediate_bin = format12(immediate);
    } else {
        immediate_bin = format12(immediate.abs());
        immediate_bin = twos_complement(&immediate_bin);
    }

    s.push_str(&immediate_bin);
    s.push_str(source_reg);
    s.push_str(instruction_bin[1]);
    s.push_str(des_reg);
    s.push_str(instruction_bin[0]);

    s
}

pub fn stype(hash_map2: &HashMap<&str, &str>, my_array: &Vec<&str>) -> String {
    let mut s: String = String::new();

    let register_bin: &str = hash_map2[my_array[1]];
    let imm_and_source_reg: Vec<&str> = my_array[2].split("(").collect();
    let source_reg: &str = imm_and_source_reg[1];
    let source_reg: &str = &source_reg.replace(")", "");

    let imm: i32 = imm_and_source_reg[0].parse().unwrap();
    let mut immediate_bin: String;
    if imm < 0 {
        immediate_bin = format12(imm.abs());
        immediate_bin = twos_complement(&immediate_bin);
    } else {
        immediate_bin = format12(imm);
    }

    s.push_str(&immediate_bin[0..7]);
    s.push_str(register_bin);
    s.push_str(hash_map2[&source_reg]);
    s.push_str("010");
    s.push_str(&immediate_bin[7..]);
    s.push_str("0100011");

    s
}

pub fn btype(
    hash_map: &HashMap<&str, Vec<&str>>,
    hash_map2: &HashMap<&str, &str>,
    my_array: &Vec<&str>,
    hash_map_labels: &HashMap<&str, i32>,
    current_line: i32,
) -> String {
    let mut s = String::new();

    let src_register1 = hash_map2[my_array[2]];
    let src_register2 = hash_map2[my_array[1]];
    let instruction = &hash_map[my_array[0]];
    // let imm: i32 = my_array[3].parse().unwrap();
    //println!("{}", my_array[3]);
    let mut flag: bool = false;
    match my_array[3].parse::<i32>() {
        Ok(_) => {
            flag = true;
        }
        Err(_) => {
            flag = false;
        }
    }

    let mut immediate_bin: String;
    if flag == true {
        let imm: i32 = my_array[3].parse().unwrap();

        if imm < 0 {
            immediate_bin = format13(imm.abs());
            immediate_bin = twos_complement(&immediate_bin);
        } else {
            immediate_bin = format13(imm);
        }
    } else {
        let imm: i32 = (hash_map_labels[my_array[3]] - current_line) * 4;

        if imm < 0 {
            immediate_bin = format13(imm.abs());
            immediate_bin = twos_complement(&immediate_bin);
        } else {
            immediate_bin = format13(imm);
        }
    }
    print!("{immediate_bin}\n");

    s.push_str(&immediate_bin[0..1]);
    s.push_str(&immediate_bin[2..8]);
    s.push_str(src_register1);
    s.push_str(src_register2);
    s.push_str(instruction[1]);
    s.push_str(&immediate_bin[8..12]);
    s.push_str(&immediate_bin[1..2]);
    s.push_str(instruction[0]);

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

    let immediate: i32 = my_array[2].parse().unwrap();
    let mut immediate_bin: String;
    if immediate >= 0 {
        immediate_bin = format32(immediate);
    } else {
        immediate_bin = format32(immediate.abs());
        immediate_bin = twos_complement(&immediate_bin);
    }
    if immediate_bin.len() < 31 {
        immediate_bin = format!("{:0>31}", immediate_bin);
    }

    s.push_str(&immediate_bin[0..20]);
    s.push_str(dest_register_bin);
    s.push_str(instruction_bin);

    s
}

pub fn jtype(
    hash_map: &HashMap<&str, &str>,
    hash_map2: &HashMap<&str, &str>,
    my_array: &Vec<&str>,
    hash_map_labels: &HashMap<&str, i32>,
    current_line: i32,
) -> String {
    let mut s: String = String::new();

    let mut flag: bool = false;
    match my_array[2].parse::<i32>() {
        Ok(_) => {
            flag = true;
        }
        Err(_) => {
            flag = false;
        }
    }

    let mut immediate_bin: String;
    if flag == true {
        let imm: i32 = my_array[2].parse().unwrap();

        if imm < 0 {
            immediate_bin = format20(imm.abs());
            immediate_bin = twos_complement(&immediate_bin);
        } else {
            immediate_bin = format20(imm);
        }
    } else {
        let imm: i32 = (hash_map_labels[my_array[2]] - current_line) * 4;

        if imm < 0 {
            immediate_bin = format20(imm.abs());
            immediate_bin = twos_complement(&immediate_bin);
        } else {
            immediate_bin = format20(imm);
        }
    }

    let instruction: &str = my_array[0];
    let instruction_bin: &str = hash_map[instruction];
    let register_bin: &str = hash_map2[my_array[1]];

    s.push_str(&immediate_bin[0..1]); //20th bit
    s.push_str(&immediate_bin[10..20]);
    s.push_str(&immediate_bin[9..10]);
    s.push_str(&immediate_bin[1..9]);
    s.push_str(register_bin);
    s.push_str(instruction_bin);
    // s.push_str(&immediate_bin[8..19]);
    // s.push_str(&immediate_bin[0..9]);
    // s.push_str(register_bin);
    // s.push_str(instruction_bin);

    s
}
