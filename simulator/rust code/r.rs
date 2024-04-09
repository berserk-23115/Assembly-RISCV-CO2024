use std::collections::HashMap;

fn main() {
    let mut registers: HashMap<&str, i32> = [
        ("00000", 1),
        ("00001", 0),
        ("00010", 0),
        ("00011", 0),
        ("00100", 0),
        ("00101", 0),
        ("00110", 0),
        ("00111", 0),
        ("01000", 0),
        ("01001", 0),
        ("01010", 0),
        ("01011", 0),
        ("01100", 0),
        ("01101", 0),
        ("01110", 0),
        ("01111", 0),
        ("10000", 0),
        ("10001", 0),
        ("10010", 0),
        ("10011", 0),
        ("10100", 0),
        ("10101", 0),
        ("10110", 0),
        ("10111", 0),
        ("11000", 0),
        ("11001", 0),
        ("11010", 0),
        ("11011", 0),
        ("11100", 0),
        ("11101", 0),
        ("11110", 0),
        ("11111", 0),
    ]
    .iter()
    .cloned()
    .collect();

    let hash_map: HashMap<&str, Vec<&str>> = [
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

    let binary_number = "00000000000000000000010010110011"; // Replace with your binary number

    let funct7 = &binary_number[0..7];
    let rs2 = &binary_number[7..12];
    println!("{}", rs2);
    let rs1 = &binary_number[12..17];
    println!("{}", rs1);
    let funct3 = &binary_number[17..20];
    let rd = &binary_number[20..25];
    println!("{}", rd);
    let optcode = &binary_number[25..32];

    for (key, values) in hash_map.iter() {
        if values[0] == optcode && values[1] == funct3 && values[2] == funct7 {
            println!("{}", key);
            matching(key, rd, rs1, rs2, &mut registers);
        }
    }
    for (key, value) in registers {
        println!("{}, {}", key, value);
    }
}

fn matching(inst: &str, rf: &str, r1: &str, r2: &str, registers: &mut HashMap<&str, i32>) {
    match inst {
        "add" => {
            *registers.get_mut(rf).unwrap() =
                *registers.get(r1).unwrap() + *registers.get(r2).unwrap();
            println!("{}", registers[rf]);
        }
        "sub" => {
            *registers.get_mut(rf).unwrap() =
                *registers.get(r1).unwrap() - *registers.get(r2).unwrap();
            println!("{}", registers[rf]);
        }
        "slt" => {
            if *registers.get(r1).unwrap() < *registers.get(r2).unwrap() {
                *registers.get_mut(rf).unwrap() = 1;
            }
        }
        "sltu" => {
            if *registers.get(r1).unwrap() < *registers.get(r2).unwrap() {
                *registers.get_mut(rf).unwrap() = 1;
            }
        }
        "xor" => {
            *registers.get_mut(rf).unwrap() =
                *registers.get(r1).unwrap() ^ *registers.get(r2).unwrap();
            println!("{}", registers[rf]);
        }
        "sll" => {
            *registers.get_mut(rf).unwrap() =
                *registers.get(r1).unwrap() * 2i32.pow(*registers.get(r2).unwrap() as u32);
            println!("{}", registers[rf]);
        }
        "srl" => {
            *registers.get_mut(rf).unwrap() =
                *registers.get(r1).unwrap() / 2i32.pow(*registers.get(r2).unwrap() as u32);
            println!("{}", registers[rf]);
        }
        "or" => {
            *registers.get_mut(rf).unwrap() =
                *registers.get(r1).unwrap() | *registers.get(r2).unwrap();
            println!("{}", registers[rf]);
        }
        "and" => {
            *registers.get_mut(rf).unwrap() =
                *registers.get(r1).unwrap() & *registers.get(r2).unwrap();
            println!("{}", registers[rf]);
        }
        _ => (),
    }
}
