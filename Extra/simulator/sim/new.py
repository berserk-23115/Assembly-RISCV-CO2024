import os
import sys


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

# def sign_extension(decimal_number, num_bits=32):
#     if decimal_number >= 0:
#         binary_number = bin(decimal_number)[2:].zfill(num_bits)
#     else:
#         binary_number = bin(2**num_bits + decimal_number)[2:]
#     return binary_number

def integer_to_binary(integer_num):
    if integer_num >= 0:
        # Convert positive integer part to binary
        binary_representation = bin(integer_num)[2:]
        return binary_representation.zfill(32)  # Add leading zero for positive numbers
    else:
        # Convert negative integer part to binary using two's complement
        binary_representation = bin(-integer_num)[2:]
        num_bits = len(binary_representation)
        # Determine the number of bits required for the two's complement representation
        num_bits_required = num_bits + 1  # Add one bit for sign
        binary_representation = binary_representation.zfill(32)
        # Flip the bits
        flipped_bits = ''.join('1' if bit == '0' else '0' for bit in binary_representation)
        # Add one to the flipped bits
        incremented_bits = bin(int(flipped_bits, 2) + 1)[2:]
        # Pad with leading zeros if necessary
        binary_representation = incremented_bits.zfill(32)
        return binary_representation

def binary_to_integer(binary_str):
    # Check if the binary number is negative
    is_negative = binary_str[0] == '1'

    # Convert binary string to integer
    integer_value = int(binary_str, 2)

    # If the number is negative, compute its two's complement
    if is_negative:
        num_bits = len(binary_str)
        integer_value -= 2 ** num_bits

    return integer_value

def twos_complement(binary_num):
    # Invert the bits
    inverted_bits = ''.join('1' if bit == '0' else '0' for bit in binary_num)

    # Add one to the inverted bits
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
            print("add")    
            registers[rf] = registers[r1] + registers[r2]
        case "sub":
            print("sub")
            registers[rf] = registers[r1] - registers[r2]
        case "sll":
            print("sll")
            registers[rf] = registers[r1] << int(integer_to_binary(registers[r2]),2)
        case "slt":
            print("slt")
            if registers[r1] < registers[r2]:
                registers[rf] = 1
        case "sltu":
            print("sltu")
            if int(integer_to_binary(registers[r1]),2) < int(integer_to_binary(registers[r2]),2):
                registers[rf] = 1
        case "xor":
            print("xor")
            registers[rf] = registers[r1] ^ registers[r2]
        case "srl":
            print("srl")
            registers[rf] = registers[r1] >> int(integer_to_binary(registers[r2]),2)
        case "or":
            print("or")
            registers[rf] = registers[r1] | registers[r2]
        case "and":
            print("and")
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
            print("beq")
            if registers[r1] == registers[r2]:
                PC += imm
            else:
                PC += 4
        case "bne":
            print("bne")
            if registers[r1] != registers[r2]:
                PC += imm
            else:
                PC += 4
        case "blt":
            print("blt")
            if registers[r1] < registers[r2]:
                PC += imm
            else:
                PC += 4
        case "bge":
            print("bge")
            if registers[r1] >= registers[r2]:
                PC += imm
            else:
                PC += 4
        case "bltu":
            print("bltu")
            if int(integer_to_binary(registers[r1]),2) < int(integer_to_binary(registers[r2]),2):
                PC += imm
            else:
                PC += 4
        case "bgeu":
            print("bgeu")
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
    file_path = os.path.abspath(
        "/home/ayush/Assembly-RISCV-CO2024/simulator/sim/input.txt"
    )
    f = open("/home/ayush/Assembly-RISCV-CO2024/simulator/sim/output.txt", "w")
    f.close()

    with open(file_path, "r") as f:
        data = f.readlines()

    global PC
    global halt
    i = 0
    
    while not halt:
        original_PC = PC
        line = data[PC//4]

        opcode = line[25:32]
        
        print(i)
        print(PC//4)
        

        match opcode:
            # R-type
            case "0110011":
                print("rtype" )
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
                print( "itype" )
                
                imm = line[0:12]
                rs1 = line[12:17]
                funct3 = line[17:20]
                rd = line[20:25]
                opcode = line[25:32]
                for keys, values in hash_map.items():
                    if values[0] == opcode and values[1] == funct3:
                        itype(keys, rd, rs1, imm)

            case "0010011":
                print("itype" )
                
                imm = line[0:12]
                rs1 = line[12:17]
                funct3 = line[17:20]
                rd = line[20:25]
                opcode = line[25:32]
                for keys, values in hash_map.items():
                    if values[0] == opcode and values[1] == funct3:
                        itype(keys, rd, rs1, imm)

            case "1100111":
                print("itype" )
                
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
                print("stype" )
                
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
                print( "btype" )
                
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
                print( "utype" )
                
                imm = line[0:20] + "000000000000"
                rd = line[20:25]
                opcode = line[25:32]
                for keys, values in hash_map.items():
                    if values[0] == opcode:
                        utype(keys, rd, imm)

            case "0010111":
                print( "utype" )
                
                imm = line[0:20] + "000000000000"
                rd = line[20:25]
                opcode = line[25:32]
                for keys, values in hash_map.items():
                    if values[0] == opcode:
                        utype(keys, rd, imm)

            # J-type
            case "1101111":
                print( "jtype" )
                
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

        i+=1
        
        # register write
        f = open("/home/ayush/Assembly-RISCV-CO2024/simulator/sim/output.txt", "a")
        f.write("0b"+decimal_to_32bit_binary(PC) + " ")
        for j in registers:
            f.write("0b"+integer_to_binary(registers[j]) + " ")
        f.write("\n")
        f.close()
        
        # register write
        f = open("/home/ayush/Assembly-RISCV-CO2024/simulator/sim/output.txt", "a")
        f.write("0b"+decimal_to_32bit_binary(PC) + " ")
        for j in registers:
            f.write("0b"+integer_to_binary(registers[j]) + " ")
        f.write("\n")
        f.close()

        if (PC//4 >= len(data) or PC == original_PC):
            halt = True

    # memory write
    f = open("/home/ayush/Assembly-RISCV-CO2024/simulator/sim/output.txt", "a")
    for i in mem:
        f.write(f"{hex_format(i)}:0b{integer_to_binary(mem[i])}")
        f.write("\n")
    print()
    f.close()


if __name__ == "__main__":
    main()
