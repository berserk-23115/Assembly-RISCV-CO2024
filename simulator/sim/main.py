import os
import sys

registers = {
    "00000": 0,
    "00001": 0,
    "00010": 0,
    "00011": 0,
    "00100": 0,
    "00101": 0,
    "00110": 0,
    "00111": 0,
    "01000": 0,
    "01001": 0,
    "01010": 0,
    "01011": 0,
    "01100": 0,
    "01101": 0,
    "01110": 0,
    "01111": 0,
    "10000": 0,
    "10001": 0,
    "10010": 0,
    "10011": 0,
    "10100": 0,
    "10101": 0,
    "10110": 0,
    "10111": 0,
    "11000": 0,
    "11001": 0,
    "11010": 0,
    "11011": 0,
    "11100": 0,
    "11101": 0,
    "11110": 0,
    "11111": 0,
}

hash_map = {
    "add": ["0110011", "000", "0000000"],
    "sub": ["0110011", "000", "0100000"],
    "sll": ["0110011", "001", "0000000"],
    "slt": ["0110011", "010", "0000000"],
    "sltu": ["0110011", "011", "0000000"],
    "xor": ["0110011", "100", "0000000"],
    "srl": ["0110011", "101", "0000000"],
    "or": ["0110011", "110", "0000000"],
    "and": ["0110011", "111", "0000000"],
    "lw": ["0000011", "010"],
    "addi": ["0010011", "000"],
    "sltiu": ["0010011", "011"],
    "jalr": ["1100111", "000"],
    "sw": ["0100011", "010"],
}
pc = 0
mem = {
    "0x00010000": 0,
    "0x00010004": 0,
    "0x00010008": 0,
    "0x0001000c": 0,
    "0x00010010": 0,
    "0x00010014": 0,
    "0x00010018": 0,
    "0x0001001c": 0,
    "0x00010020": 0,
    "0x00010024": 0,
    "0x00010028": 0,
    "0x0001002c": 0,
    "0x00010030": 0,
    "0x00010034": 0,
    "0x00010038": 0,
    "0x0001003c": 0,
    "0x00010040": 0,
    "0x00010044": 0,
    "0x00010048": 0,
    "0x0001004c": 0,
    "0x00010050": 0,
    "0x00010054": 0,
    "0x00010058": 0,
    "0x0001005c": 0,
    "0x00010060": 0,
    "0x00010064": 0,
    "0x00010068": 0,
    "0x0001006c": 0,
    "0x00010070": 0,
    "0x00010074": 0,
    "0x00010078": 0,
    "0x0001007c": 0,
}


def convertion(y):
    if y[0] == "0":
        con = int(y, 2)
        return con
    elif y[0] == "1":
        y = list(y)
        for i in range(12):
            if y[i] == "0":
                y[i] = "1"
            else:
                y[i] = "0"
        y = "".join(y)
        con = int(y, 2)
        con = con + 1
        fin = con - (con * 2)
        return fin


def signed_binaryToDecimal(binary, sign):

    decimal, i = 0, 0
    if sign == 0:
        while binary != 0:
            dec = binary % 10
            decimal = decimal + dec * pow(2, i)
            binary = binary // 10
            i += 1
        return decimal
    else:
        binary = binary % 1000000000000
        while binary != 0:
            dec = binary % 10
            decimal = decimal + dec * pow(2, i)
            binary = binary // 10
            i += 1
        return decimal * (-1)


def binaryToDecimal(binary):

    decimal, i = 0, 0
    while binary != 0:
        dec = binary % 10
        decimal = decimal + dec * pow(2, i)
        binary = binary // 10
        i += 1
    return decimal


def int_to_32(x):
    if x >= 0:
        y = format(x, '032b')
        return y
        print(y)
    elif x < 0:
        y = format(x + 1, '032b')
        # print(y)
        y = list(y)
        for i in range(32):
            if y[i] == "0":
                y[i] = "1"
            else:
                y[i] = "0"
        y[0] = "1"
        y = ''.join(y)
        return y


def convertion2(y):
    return int(y, 2)


def matching_rtype(inst, rf, r1, r2):
    match inst:
        case "add":
            registers[rf] = registers[r1] + registers[r2]
            print(registers[rf])
        case "sub":
            registers[rf] = registers[r1] - registers[r2]
            print(registers[rf])
        case "slt":
            if registers[r1] < registers[r2]:
                registers[rf] = 1
        case "sltu":
            if registers[r1] < registers[r2]:
                registers[rf] = 1
        case "xor":
            registers[rf] = registers[r1] ^ registers[r2]
            print(registers[rf])
        case "sll":
            registers[rf] = registers[r1] * pow(2, registers[r2])
            print(registers[rf])
        case "slr":
            registers[rf] = registers[r1] / pow(2, registers[r2])
            print(registers[rf])
        case "or":
            registers[rf] = registers[r1] | registers[r2]
            print(registers[rf])
        case "and":
            registers[rf] = registers[r1] & registers[r2]
            print(registers[rf])


def matching_itype(inst, rf, r1, imm, pc):
    match inst:
        case "lw":
            registers[rf] = mem[registers[r1] + convertion(imm)]
            pc += 4
            print(registers[rf])
        case "addi":
            registers[rf] = registers[r1] + convertion(imm)
            pc += 4
            print(registers[rf])
        case "sltiu":
            if registers[r1] < convertion(imm):
                registers[rf] = 1
                print(registers[rf])
            pc += 4
        case "jalr":
            registers[rf] = pc[0] + 4
            pc = registers[rs1] + convertion(imm)
    return pc


def matching_stype(inst, rf, r1, imm):
    match inst:
        case "sw":
            mem[hex(registers[r1] + convertion(imm))] = registers[rf]


def btype_s(my_array, registers, pc):
    imm = my_array[0:1] + my_array[26:27] + \
        my_array[2:8] + my_array[21:25] + "0"
    rs2 = my_array[8:13]
    rs1 = my_array[13:18]
    func3 = my_array[18:21]
    match func3:
        case "000":
            if registers[rs1] == registers[rs2]:
                imm = binaryToDecimal(int(imm))
                pc += imm
            else:
                pc += 4
        case "001":
            if registers[rs1] != registers[rs2]:
                imm = binaryToDecimal(int(imm))
                pc += imm
            else:
                pc += 4
        case "100":
            if registers[rs1] >= registers[rs2]:
                imm = binaryToDecimal(int(imm))
                pc += imm
            else:
                pc += 4
        case "101":
            if abs(registers[rs1]) >= abs(registers[rs2]):
                imm = signed_binaryToDecimal(int(imm), int(imm[0]))
                pc += imm
            else:
                pc += 4
        case "110":
            if registers[rs1] < registers[rs2]:
                imm = binaryToDecimal(int(imm))
                pc += imm
            else:
                pc += 4
        case "111":
            if abs(registers[rs1]) < abs(registers[rs2]):
                imm = binaryToDecimal(int(imm))
                pc += imm
            else:
                pc += 4
    return pc


def matching_bonus(inst, rf, r1, r2):
    match inst:
        case "mul":
            registers[rf] = registers[r1] * registers[r2]
            print(registers[rf])
        case "rst":
            for i in registers:
                registers[i] = 0
        case "halt":
            pass
        case "rvrs":
            print(registers[rf])
            x = int_to_32(registers[rf])
            print(x)
            z = convertion2(x[::-1])
            print(z)
            registers[r1] = z


def write_reg():
    f = open("/home/ayush/Assembly-RISCV-CO2024/simulator/sim/output.txt", "a")
    for i in registers:
        f.write(str(registers[i]))
    f.write("\n")
    f.close()


def write_mem():
    f = open("/home/ayush/Assembly-RISCV-CO2024/simulator/sim/output.txt", "a")
    for i in mem:
        f.write(f"{i}:{mem[i]}")
    print()
    f.close()


if __name__ == "__main__":
    # FIXME: file path
    file_path = os.path.abspath(
        "/home/ayush/Assembly-RISCV-CO2024/simulator/sim/input.txt"
    )
    print(file_path)

    with open(file_path, "r") as f:
        data = f.readlines()

    # print(data)

    while True:
        # TODO: break loop logic

        line = data[pc // 4]
        opcode = line[-7:]
        print(opcode)

        if opcode == "0110011":
            print("R-type")
            funct7 = line[0:7]
            rs2 = line[7:12]
            rs1 = line[12:17]
            funct3 = line[17:20]
            rd = line[20:25]
            opcode = line[25:32]

            matching_rtype(hash_map[opcode][0], rd, rs1, rs2)
            pc += 4

        elif opcode == "0000011":
            print("I-type")
            imm = line[0:12]
            rs1 = line[12:17]
            funct3 = line[17:20]
            rd = line[20:25]
            opcode = line[25:32]

            pc = matching_itype(hash_map[opcode][0], rd, rs1, imm, pc)

        elif opcode == "0100011":
            print("S-type")
            imm1 = line[0:7]
            rs1 = line[12:17]
            rd = line[7:12]
            funct3 = line[17:20]
            imm2 = line[20:25]
            imm = imm2 + imm1

            matching_stype(hash_map[opcode][0], rd, rs1, imm)
            pc += 4

        elif opcode == "1100011":
            print("B-type")
            pc = btype_s(line, registers, pc)

        elif opcode == "0110111":
            print("U-type")
            # TODO: U-type logic
        elif opcode == "1101111":
            print("J-type")
            # TODO: J-type logic
        elif opcode == "0000001":
            print("BONUS")
            funct7 = line[0:7]
            rs2 = line[7:12]
            rs1 = line[12:17]
            funct3 = line[17:20]
            rd = line[20:25]
            optcode = line[25:32]
            
            match inst:
                case "mul":
                    registers[rd] = registers[rs1] * registers[rs2]
                    print(registers[rd])
                case "rst":
                    for i in registers:
                        registers[i] = 0
                case "halt":
                    break
                case "rvrs":
                    print(registers[rd])
                    x = int_to_32(registers[rd])
                    print(x)
                    z = convertion2(x[::-1])
                    print(z)
                    registers[rs1] = z

        if (pc/4 >= len(data)):
            break

        # write_reg()
        print(registers)

    write_mem()
