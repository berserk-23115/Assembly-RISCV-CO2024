import os
import sys

file_name = sys.argv[1]
output_name = sys.argv[2]

file = open(file_name, 'r')
data = file.readlines()
outfile = open(output_name, 'w')

registers = {format(i, '05b'): 0 for i in range(32)}
registers["00010"] = 256

hash_map = {
    # R-type
    "add": ["0110011", "000", "0000000"],
    "sub": ["0110011", "000", "0100000"],
    "sll": ["0110011", "001", "0000000"],
    "slt": ["0110011", "010", "0000000"],
    "sltu": ["0110011", "011", "0000000"],
    "xor": ["0110011", "100", "0000000"],
    "srl": ["0110011", "101", "0000000"],
    "or": ["0110011", "110", "0000000"],
    "and": ["0110011", "111", "0000000"],
    # I-type
    "lw": ["0000011", "010"],
    "addi": ["0010011", "000"],
    "sltiu": ["0010011", "011"],
    "jalr": ["1100111", "000"],
    # S-type
    "sw": ["0100011", "010"],
    # B-type
    "beq": ["1100011", "000"],
    "bne": ["1100011", "001"],
    "blt": ["1100011", "100"],
    "bge": ["1100011", "101"],
    "bltu": ["1100011", "110"],
    "bgeu": ["1100011", "111"],
    # U-type
    "lui": ["0110111"],
    "auipc": ["0010111"],
    # J-type
    "jal": ["1101111"],
    # BONUS
    "mul": ["0000000", "000", "0000001"],
    "rst": ["0000000", "000", "0000010"],
    "halt": ["0000000", "000", "0000011"],
    "rvrs": ["0000000", "000", "0000100"]
}

mem = {
    "0x10000": 0,
    "0x10004": 0,
    "0x10008": 0,
    "0x1000c": 0,
    "0x10010": 0,
    "0x10014": 0,
    "0x10018": 0,
    "0x1001c": 0,
    "0x10020": 0,
    "0x10024": 0,
    "0x10028": 0,
    "0x1002c": 0,
    "0x10030": 0,
    "0x10034": 0,
    "0x10038": 0,
    "0x1003c": 0,
    "0x10040": 0,
    "0x10044": 0,
    "0x10048": 0,
    "0x1004c": 0,
    "0x10050": 0,
    "0x10054": 0,
    "0x10058": 0,
    "0x1005c": 0,
    "0x10060": 0,
    "0x10064": 0,
    "0x10068": 0,
    "0x1006c": 0,
    "0x10070": 0,
    "0x10074": 0,
    "0x10078": 0,
    "0x1007c": 0,
}

PC = 0
halt = False


def unsigned_binary_to_decimal(binary):
    return int(binary, 2)


def decimal_to_32bit_binary(decimal):
    if decimal >= 0:
        binary = format(decimal, '032b')
        return binary
    elif decimal < 0:
        binary = format(decimal + 1, '032b')
        binary = list(binary)
        for i in range(32):
            if binary[i] == "0":
                binary[i] = "1"
            else:
                binary[i] = "0"
        binary[0] = "1"
        binary = ''.join(binary)
        return binary

def signed_binary_to_decimal(binary):
    if binary[0] == "0":
        return unsigned_binary_to_decimal(binary)
    else:
        binary = list(binary)
        for i in range(32):
            if binary[i] == "0":
                binary[i] = "1"
            else:
                binary[i] = "0"
        binary[0] = "0"
        binary = ''.join(binary)
        return -1 * (unsigned_binary_to_decimal(binary) + 1)

def integer_to_binary(integer_num):
    if integer_num >= 0:
        binary_representation = bin(integer_num)[2:]
        return binary_representation.zfill(32) 
    else:
        binary_representation = bin(-integer_num)[2:]
        num_bits = len(binary_representation)
        num_bits_required = num_bits + 1 
        binary_representation = binary_representation.zfill(32)
        flipped_bits = ''.join('1' if bit == '0' else '0' for bit in binary_representation)
        incremented_bits = bin(int(flipped_bits, 2) + 1)[2:]
        binary_representation = incremented_bits.zfill(32)
        return binary_representation

def binary_to_integer(binary_str):
    is_negative = binary_str[0] == '1'
    integer_value = int(binary_str, 2)
    if is_negative:
        num_bits = len(binary_str)
        integer_value -= 2 ** num_bits

    return integer_value

def twos_complement(binary_num):
    inverted_bits = ''.join('1' if bit == '0' else '0' for bit in binary_num)
    carry = 1
    result_bits = ''
    for bit in inverted_bits[::-1]:
        if bit == '1' and carry == 1:
            result_bits = '0' + result_bits
        elif bit == '0' and carry == 1:
            result_bits = '1' + result_bits
            carry = 0
        else:
            result_bits = bit + result_bits

    return result_bits


def rtype(inst, rf, r1, r2):
    match inst:
        case "add":
            registers[rf] = registers[r1] + registers[r2]
        case "sub":
            print("sub")
            registers[rf] = registers[r1] - registers[r2]
        case "sll":
            registers[rf] = registers[r1] << int(integer_to_binary(registers[r2]),2)
        case "slt":
            if registers[r1] < registers[r2]:
                registers[rf] = 1
        case "sltu":
            if int(integer_to_binary(registers[r1]),2) < int(integer_to_binary(registers[r2]),2):
                registers[rf] = 1
        case "xor":
            registers[rf] = registers[r1] ^ registers[r2]
        case "srl":
            registers[rf] = registers[r1] >> int(integer_to_binary(registers[r2]),2)
        case "or":
            registers[rf] = registers[r1] | registers[r2]
        case "and":
            registers[rf] = registers[r1] & registers[r2]
    global PC
    PC += 4


def itype(inst, rf, r1, imm):
    global PC
    match inst:
        case "lw":
            registers[rf] = mem[hex(registers[r1] + binary_to_integer(imm))]
            PC += 4
        case "addi":
            registers[rf] = registers[r1] + binary_to_integer(imm)
            PC += 4
        case "sltiu":
            if int(registers[r1],2) < int(imm,2):
                registers[rf] = 1
            PC += 4
        case "jalr":
            registers[rf] = PC + 4
            PC = registers[rf] + binary_to_integer(imm)
            PC = PC & 0xFFFFFFFE


def stype(_, r1, r2, imm):
    mem[hex(registers[r1] + binary_to_integer(imm))] = registers[r2]
    global PC
    PC += 4


def btype(inst, r1, r2, imm):
    global PC
    imm = binary_to_integer(imm)
    match inst:
        case "beq":
            if registers[r1] == registers[r2]:
                PC += imm
            else:
                PC += 4
        case "bne":
            if registers[r1] != registers[r2]:
                PC += imm
            else:
                PC += 4
        case "blt":
            if registers[r1] < registers[r2]:
                PC += imm
            else:
                PC += 4
        case "bge":
            if registers[r1] >= registers[r2]:
                PC += imm
            else:
                PC += 4
        case "bltu":
            if int(integer_to_binary(registers[r1]),2) < int(integer_to_binary(registers[r2]),2):
                PC += imm
            else:
                PC += 4
        case "bgeu":
            if int(integer_to_binary(registers[r1]),2) >= int(integer_to_binary(registers[r2]),2):
                PC += imm
            else:
                PC += 4


def utype(inst, rf, imm):
    global PC
    imm = binary_to_integer(imm)
    match inst:
        case "lui":
            registers[rf] = imm
        case "auipc":
            registers[rf] = PC + imm
    PC += 4


def jtype(inst, rf, imm):
    global PC
    imm = binary_to_integer(imm)
    match inst:
        case "jal":
            registers[rf] = PC + 4
            PC += imm
            PC = PC & 0xFFFFFFFE


def bonus(inst, rf, r1, r2):
    match inst:
        case "mul":
            registers[rf] = registers[r1] * registers[r2]
        case "rst":
            for i in registers:
                registers[i] = 0
        case "halt":
            global halt
            halt = True
        case "rvrs":
            x = decimal_to_32bit_binary(registers[rf])
            z = unsigned_binary_to_decimal(x[::-1])
            registers[r1] = z


def hex_format(str):
    x = str[2:]
    return "0x" + (10-len(str))*"0" + x


def main():

    global PC
    global halt
    i = 0
    
    while not halt:
        original_PC = PC
        line = data[PC//4]

        opcode = line[25:32]
        
        match opcode:
            # R-type
            case "0110011":
                funct7 = line[0:7]
                rs2 = line[7:12]
                rs1 = line[12:17]
                funct3 = line[17:20]
                rd = line[20:25]
                opcode = line[25:32]
                for keys, values in hash_map.items():
                    if values[0] == opcode and values[1] == funct3 and values[2] == funct7:
                        rtype(keys, rd, rs1, rs2)

            # I-type
            case "0000011":
                imm = line[0:12]
                rs1 = line[12:17]
                funct3 = line[17:20]
                rd = line[20:25]
                opcode = line[25:32]
                for keys, values in hash_map.items():
                    if values[0] == opcode and values[1] == funct3:
                        itype(keys, rd, rs1, imm)

            case "0010011":
                imm = line[0:12]
                rs1 = line[12:17]
                funct3 = line[17:20]
                rd = line[20:25]
                opcode = line[25:32]
                for keys, values in hash_map.items():
                    if values[0] == opcode and values[1] == funct3:
                        itype(keys, rd, rs1, imm)

            case "1100111":
                imm = line[0:12]
                rs1 = line[12:17]
                funct3 = line[17:20]
                rd = line[20:25]
                opcode = line[25:32]
                for keys, values in hash_map.items():
                    if values[0] == opcode and values[1] == funct3:
                        itype(keys, rd, rs1, imm)

            # S-type
            case "0100011":
                imm = line[0:7] + line[20:25]
                rs2 = line[7:12]
                rs1 = line[12:17]
                funct3 = line[17:20]
                opcode = line[25:32]
                for keys, values in hash_map.items():
                    if values[0] == opcode and values[1] == funct3:
                        stype(keys, rs1, rs2, imm)

            # B-type
            case "1100011":
                imm = line[0] + line[24] + line[1:7] + line[20:24] + "0"
                rs2 = line[7:12]
                rs1 = line[12:17]
                funct3 = line[17:20]
                opcode = line[25:32]
                for keys, values in hash_map.items():
                    if values[0] == opcode and values[1] == funct3:
                        btype(keys, rs1, rs2, imm)

            # U-type
            case "0110111":
                imm = line[0:20] + "000000000000"
                rd = line[20:25]
                opcode = line[25:32]
                for keys, values in hash_map.items():
                    if values[0] == opcode:
                        utype(keys, rd, imm)

            case "0010111":
                imm = line[0:20] + "000000000000"
                rd = line[20:25]
                opcode = line[25:32]
                for keys, values in hash_map.items():
                    if values[0] == opcode:
                        utype(keys, rd, imm)

            # J-type
            case "1101111":
                imm = line[0] + line[12:20] + line[11] + line[1:11] + "0"
                rd = line[20:25]
                opcode = line[25:32]
                for keys, values in hash_map.items():
                    if values[0] == opcode:
                        jtype(keys, rd, imm)

            # BONUS
            case "0000000":
                funct7 = line[0:7]
                rs2 = line[7:12]
                rs1 = line[12:17]
                funct3 = line[17:20]
                rd = line[20:25]
                opcode = line[25:32]
                for keys, values in hash_map.items():
                    if values[0] == opcode and values[1] == funct3 and values[2] == funct7:
                        bonus(keys, rd, rs1, rs2)

        
        global outfile
        outfile.write("0b"+decimal_to_32bit_binary(PC) + " ")
        for j in registers:
            outfile.write("0b"+integer_to_binary(registers[j]) + " ")
        outfile.write("\n")

        if (PC//4 >= len(data) or PC == original_PC):
            halt = True

    for i in mem:
        outfile.write(f"{hex_format(i)}:0b{integer_to_binary(mem[i])}")
        outfile.write("\n")


if __name__ == "__main__":
    main()
