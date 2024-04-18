use std::collections::HashMap;
use helper_functions::*;

pub fn utype_up(hash_map : &HashMap<&str,&str>,
    input: String){
        let mut opcode String = input[25..31];
        if opcode == "0110111"{
                let mut immediate_bin : String = input[0..20];
                let mut register_bin : &str = input[20..25];
                let mut final_bin : String = String :: new();
                final_bin.push_str(string : & immediate_bin);
                final_bin.push_str("000000000000");
                //updation of the regsiter meemory
                hash_map[&register_bin]=final_bin;

        }
        // Lui
        else if opcode == "0010111" {
                let mut program_counter : i32 = hash_map["PC"].parse().unwrap();
                let mut immediate_bin : String = input[0..20];
                let mut register_bin : &str = input[20..25];
                let mut final_bin : String = String :: new();
                final_bin.push_str(string : & immediate_bin);
                final_bin.push_str("000000000000");

                let mut add_operand : i32 = final_bin.parse().unwrap();
                let final_num : i32 = add_operand+program_counter;
                if final_num >= 0 {
                    hash_map[&register_bin] = format32(final_num);
                }
                else{ 
                    immediate_bin = format32(final_num.abs());
                    immediate_bin = twos_complement(&immediate_bin);
                    hash_map[&register_bin] = format32(final_num);
                }
        } 
        //aupic
        let mut count_int i32 = hash_map["PC"].parse().unwrap();
        count_int+=4;
        hash_map["PC"]= format32(count_int);

}

fn main(){
    let register_output : HashMap<&str,&str> = [
        //Program Counter
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

    ].clone().iter().collect();
    
}

