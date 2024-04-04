use std::collections::HashMap;

fn main() {
    let mut register_output: HashMap<&str, String> = [
        // Program Counter
        ("PC", "00000000000000000000000000000000".to_string()),
        // Registers and Their Stored Value
        ("00000", "00000000000000000000000000000000".to_string()),
        ("00001", "00000000000000000000000000000000".to_string()),
        ("00010", "00000000000000000000000000000000".to_string()),
        ("00011", "00000000000000000000000000000000".to_string()),
        ("00100", "00000000000000000000000000000000".to_string()),
        ("00101", "00000000000000000000000000000000".to_string()),
        ("00110", "00000000000000000000000000000000".to_string()),
        ("00111", "00000000000000000000000000000000".to_string()),
        ("01000", "00000000000000000000000000000000".to_string()),
        ("01001", "00000000000000000000000000000000".to_string()),
        ("01010", "00000000000000000000000000000000".to_string()),
        ("01011", "00000000000000000000000000000000".to_string()),
        ("01100", "00000000000000000000000000000000".to_string()),
        ("01101", "00000000000000000000000000000000".to_string()),
        ("01110", "00000000000000000000000000000000".to_string()),
        ("01111", "00000000000000000000000000000000".to_string()),
        ("10000", "00000000000000000000000000000000".to_string()),
        ("10001", "00000000000000000000000000000000".to_string()),
        ("10010", "00000000000000000000000000000000".to_string()),
        ("10011", "00000000000000000000000000000000".to_string()),
        ("10100", "00000000000000000000000000000000".to_string()),
        ("10101", "00000000000000000000000000000000".to_string()),
        ("10110", "00000000000000000000000000000000".to_string()),
        ("10111", "00000000000000000000000000000000".to_string()),
        ("11000", "00000000000000000000000000000000".to_string()),
        ("11001", "00000000000000000000000000000000".to_string()),
        ("11010", "00000000000000000000000000000000".to_string()),
        ("11011", "00000000000000000000000000000000".to_string()),
        ("11100", "00000000000000000000000000000000".to_string()),
        ("11101", "00000000000000000000000000000000".to_string()),
        ("11110", "00000000000000000000000000000000".to_string()),
        ("11111", "00000000000000000000000000000000".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    let mut input = "00000000000000010000010000110111".to_string();
    println!(
        "{},{},{},{}",
        input,
        &input[0..20],
        &input[20..25],
        &input[25..31]
    );
    utype_up(&mut register_output, input);

    for (key, value) in &register_output {
        println!("{}, {}", key, value);
    }
}
pub fn utype_up(hash_map: &mut HashMap<&str, String>, input: String) {
    let mut opcode = &input[25..31];
    if opcode == "0110111" {
        let mut immediate_bin = &input[0..20];
        let mut register_bin: &str = &input[20..25];
        let mut final_bin = String::new();
        final_bin.push_str(immediate_bin);
        final_bin.push_str("000000000000");
        //updation of the regsiter meemory
        hash_map.insert(register_bin, final_bin);
    }

    // Lui

    // let mut count_int i32 = hash_map["PC"].parse().unwrap();
    // count_int+=4;
    // hash_map.insert("PC", &format!("{:032b}", count_int))
}