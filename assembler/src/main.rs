mod helper_functions;
mod instructions;

use helper_functions::*;
use instructions::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

fn main() {
    let instructions: Vec<Vec<&str>> = vec![
        vec![
            "add", "sub", "sll", "slt", "sltu", "xor", "srl", "or", "and",
        ],
        vec!["lw", "addi", "sltiu", "jalr"],
        vec!["sw"],
        vec!["beq", "bne", "blt", "bge", "bltu", "bgeu"],
        vec!["lui", "auipc"],
        vec!["jal"],
        vec!["mul", "rst", "halt", "rvrs"],
    ];

    let hash_map_R: HashMap<&str, Vec<&str>> = [
        ("add", vec!["0110011", "000", "0000000"]),
        ("sub", vec!["0110011", "000", "0100000"]),
        ("sll", vec!["0110011", "001", "0000000"]),
        ("slt", vec!["0110011", "010", "0000000"]),
        ("sltu", vec!["0110011", "011", "0000000"]),
        ("xor", vec!["0110011", "100", "0000000"]),
        ("srl", vec!["0110011", "101", "0000000"]),
        ("or", vec!["0110011", "110", "0000000"]),
        ("and", vec!["0110011", "111", "0000000"]),
    ]
    .iter()
    .cloned()
    .collect();

    let hash_map_I: HashMap<&str, Vec<&str>> = [
        ("addi", vec!["0010011", "000"]),
        ("sltiu", vec!["0010011", "011"]),
        ("jalr", vec!["1100111", "000"]),
    ]
    .iter()
    .cloned()
    .collect();

    let hash_map_B: HashMap<&str, Vec<&str>> = [
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

    let hash_map_U: HashMap<&str, &str> = [("lui", "0110111"), ("auipc", "0010111")]
        .iter()
        .cloned()
        .collect();

    let hash_map_J: HashMap<&str, &str> = [("jal", "1101111")].iter().cloned().collect();

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
    let mut hash_map_labels: HashMap<&str, i32> = HashMap::new();
    let mut code_line: i32 = 0;
    let label: &str = "";
    for line in contents.lines() {
        let mut my_array: Vec<&str> = Vec::new();
        // let label: Option<&str> = None;
        code_line = code_line + 1;
        for word in line.split_whitespace() {
            let parts: Vec<&str> = word.split(',').collect();
            for part in parts {
                if part.ends_with(':') {
                    // label = part;
                    // let label: &str = &label.replace(":", "");
                    hash_map_labels.insert(&part[..(part.len() - 1)], code_line);
                } else {
                    my_array.push(part);
                }
            }
        }

        // if let Some(l) = label {
        //     my_array.push(l);
        // }
        arr.push(my_array);
    }

    print!("{:?}", hash_map_labels);
    // print!("{:?}", arr[arr.len() - 1]);

    if is_syntax_error(arr.clone()) || arr[arr.len() - 1] != vec!["beq", "zero", "zero", "0"] {
        print!("Syntax error: Virtual Hault Not found");
        exit(0);
    }

    let mut machine_code: Vec<String> = vec![];
    let mut current_line: i32 = 0;
    for line in arr {
        current_line += 1;
        if instructions[0].contains(&line[0]) {
            machine_code.push(rtype(&hash_map_R, &hash_map2, &line));
        } else if instructions[1].contains(&line[0]) {
            machine_code.push(itype(&hash_map_I, &hash_map2, &line));
        } else if instructions[2].contains(&line[0]) {
            machine_code.push(stype(&hash_map2, &line))
        } else if instructions[3].contains(&line[0]) {
            machine_code.push(btype(
                &hash_map_B,
                &hash_map2,
                &line,
                &hash_map_labels,
                current_line,
            ))
        } else if instructions[4].contains(&line[0]) {
            machine_code.push(utype(&hash_map_U, &hash_map2, &line));
        } else if instructions[5].contains(&line[0]) {
            machine_code.push(jtype(
                &hash_map_J,
                &hash_map2,
                &line,
                &hash_map_labels,
                current_line,
            ));
        }
    }

    // for line in machine_code {
    //     print!("\n{line}\n");
    // }
    let filename = "output.txt";

    match write_output(filename, &machine_code.clone()) {
        Ok(_) => println!("Contents written to {}", filename),
        Err(err) => eprintln!("Error writing to file: {}", err),
    }
}
