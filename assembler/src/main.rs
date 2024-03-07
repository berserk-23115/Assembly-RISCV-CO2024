use std::collections::HashMap;

fn main() {
    // opcodes
    let mut opcodes: HashMap<&str, &str> = HashMap::new();
    opcodes.insert("lw", "0000011");
    opcodes.insert("addi", "0010011");
    opcodes.insert("sltiu", "0010011");
    opcodes.insert("jalr", "1100111");

    // registers
    let mut registers: HashMap<&str, &str> = HashMap::new();
    registers.insert("r0", "00000");
    registers.insert("r1", "00001");
    registers.insert("r2", "00010");
    registers.insert("r3", "00011");
    registers.insert("r4", "00100");
    registers.insert("r5", "00101");
    registers.insert("r6", "00110");
    registers.insert("r7", "00111");
    registers.insert("r8", "01000");
    registers.insert("r9", "01001");
    registers.insert("r10", "01010");
    registers.insert("r11", "01011");
    registers.insert("r12", "01100");
    registers.insert("r13", "01101");
    registers.insert("r14", "01110");
    registers.insert("r15", "01111");
    registers.insert("r16", "10000");
    registers.insert("r17", "10001");
    registers.insert("r18", "10010");
    registers.insert("r19", "10011");
    registers.insert("r20", "10100");
    registers.insert("r21", "10101");
    registers.insert("r22", "10110");
    registers.insert("r23", "10111");
    registers.insert("r24", "11000");
    registers.insert("r25", "11001");
    registers.insert("r26", "11010");
    registers.insert("r27", "11011");
    registers.insert("r28", "11100");
    registers.insert("r29", "11101");
    registers.insert("r30", "11110");
    registers.insert("r31", "11111");
}
