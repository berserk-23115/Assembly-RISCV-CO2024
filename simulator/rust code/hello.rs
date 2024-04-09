// use std::collections::HashMap;
// use helper_functions::*;
// pub fn u_type_up(hash_map: &mut std::collections::HashMap<String, String>, input: String) {
//     if input.len() >= 31 {
//         let opcode: String = input[25..31].to_string();
//         if opcode == "0110111" {
//             let immediate_bin: String = input[0..20].to_string();
//             let register_bin: &str = &input[20..25];
//             let mut final_bin: String = String::new();
//             final_bin.push_str(&immediate_bin);
//             final_bin.push_str("000000000000");
//             hash_map.insert(register_bin.to_string(), final_bin);
//         } else if opcode == "0010111" {
//             let program_counter: i32 = hash_map["PC"].parse().unwrap();
//             let immediate_bin: String = input[0..20].to_string();
//             let register_bin: &str = &input[20..25];
//             let mut final_bin: String = String::new();
//             final_bin.push_str(&immediate_bin);
//             final_bin.push_str("000000000000");
//             let add_operand: i32 = final_bin.parse().unwrap();
//             let final_num: i32 = add_operand + program_counter;
//             if final_num >= 0 {
//                 hash_map.insert(register_bin.to_string(), final_num.to_string());
//             }
//         }
//     } else {
//         println!("Input string is too short");
//     }
// }


use std::collections::HashMap;

pub fn u_type_up(hash_map: &mut HashMap<String, String>, input: String) {
    let opcode = &input[25..31];
    if opcode == "0110111" {
        let immediate_bin = &input[0..20];
        let register_bin: String = input[20..25].to_string();
        let mut final_bin = String::new();
        final_bin.push_str(immediate_bin);
        final_bin.push_str("000000000000");
        //updation of the register memory
        hash_map.insert(register_bin, final_bin);
    }

    // Lui
    if opcode == "YOUR_OPCODE_FOR_LUI" { // Replace with your opcode for LUI
        // Your implementation here
    }

    // Increment "PC"
    if let Some(pc) = hash_map.get_mut("PC") {
        if let Ok(mut count_int) = i32::from_str_radix(pc, 2) {
            count_int += 4;
            *pc = format!("{:032b}", count_int);
        }
    }
}

fn main() {
    let mut map: HashMap<String, String> = [
        ("PC","00000000000000000000000000000000"),

        //Registers and Their Stored Value
        ("00000","00000000000000000000000000000000"),
        ("00001","00000000000000000000000000000000"),
        ("00010","00000000000000000000000000000000"),
        ("00011","00000000000000000000000000000000"),
        ("00100","00000000000000000000000000000000"),
        ("00101","00000000000000000000000000000000"),
        ("00110","00000000000000000000000000000000"),
        ("00111","00000000000000000000000000000000"),
        ("01000","00000000000000000000000000000000"),
        ("01001","00000000000000000000000000000000"),
        ("01010","00000000000000000000000000000000"),
        ("01011","00000000000000000000000000000000"),
        ("01100","00000000000000000000000000000000"),
        ("01101","00000000000000000000000000000000"),
        ("01110","00000000000000000000000000000000"),
        ("01111","00000000000000000000000000000000"),
        ("10000","00000000000000000000000000000000"),
        ("10001","00000000000000000000000000000000"),
        ("10010","00000000000000000000000000000000"),
        ("10011","00000000000000000000000000000000"),
        ("10100","00000000000000000000000000000000"),
        ("10101","00000000000000000000000000000000"),
        ("10110","00000000000000000000000000000000"),
        ("10111","00000000000000000000000000000000"),
        ("11000","00000000000000000000000000000000"),
        ("11001","00000000000000000000000000000000"),
        ("11010","00000000000000000000000000000000"),
        ("11011","00000000000000000000000000000000"),
        ("11100","00000000000000000000000000000000"),
        ("11101","00000000000000000000000000000000"),
        ("11110","00000000000000000000000000000000"),
        ("11111","00000000000000000000000000000000"),

    ].iter().cloned().collect();

    let input : String = "00000000000000010000010000110111".to_string();
    u_type_up(&mut map, input);
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}
