use std::collections::HashMap;

fn main() {
    let mut registers: HashMap<&str, i32> = [
        ("00000", 0),
        ("00001", 0),
        ("00010", 0),
        ("00011", 0),
        ("00100", 0),
        ("00101", 0),
        ("00110", 0),
        ("00111", 0),
        ("01000", 65536),
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

    let mut pc = vec![0];
    let mut mem = HashMap::new();

    let hash_map: HashMap<&str, Vec<&str>> = [
        ("sw", vec!["0100011", "010"]),
        // Add more entries if needed
    ]
    .iter()
    .cloned()
    .collect();

    fn conversion(y: &str) -> i32 {
        if y.chars().next().unwrap() == '0' {
            return i32::from_str_radix(y, 2).unwrap();
        } else {
            let mut y = y.chars().collect::<Vec<char>>();
            for i in 0..y.len() {
                if y[i] == '0' {
                    y[i] = '1';
                } else {
                    y[i] = '0';
                }
            }
            let y = y.into_iter().collect::<String>();
            let mut con = i32::from_str_radix(&y, 2).unwrap();
            con += 1;
            let fin = con - (con * 2);
            return fin;
        }
    }

    fn matching(
        inst: &str,
        rf: &str,
        r1: &str,
        imm: &str,
        registers: &mut HashMap<&str, i32>,
        mem: &mut HashMap<String, i32>,
    ) {
        match inst {
            "sw" => {
                let key = format!("{:x}", registers[r1] + conversion(imm));
                mem.insert(key, *registers.get(rf).unwrap());
            }
            _ => (),
        }
        for (key, value) in mem.iter() {
            println!("{} {}", key, value);
        }
    }

    let binary_number = String::from("00000000100101000010000000100011");

    let imm1 = &binary_number[0..7];
    println!("{}", imm1);
    let rs1 = &binary_number[12..17];
    let rd = &binary_number[7..12];
    println!("{}", rd);
    println!("{}", rs1);

    let funct3 = &binary_number[17..20];
    let imm2 = &binary_number[20..25];

    let imm = format!("{}{}", imm2, imm1);
    println!("{}", imm);
    println!("{}", conversion(&imm));

    let optcode = &binary_number[25..32];

    for (key, values) in hash_map.iter() {
        if values[0] == optcode && values[1] == funct3 {
            println!("{}", key);
            matching(key, rd, rs1, &imm, &mut registers, &mut mem);
        }
    }
    for (key, value) in &registers {
        println!("{}, {}", key, value);
    }
}
