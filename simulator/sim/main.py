import os
import sys


registers = {format(i, '05b'): 0 for i in range(32)}
registers["00010"] = 256
print(registers)

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


def hex_format(str):
    x = str[2:]
    return "0x" + (10-len(str))*"0" + x


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


def matching_rtype(inst, rf, r1, r2, registers=registers, mem=mem):
    match inst:
        case "add":
            registers[rf] = registers[r1] + registers[r2]
            print(registers[rf])
        case "sub":
            registers[rf] = registers[r1] - registers[r2]
            # print(registers[rf])
        case "slt":
            if registers[r1] < registers[r2]:
                registers[rf] = 1
        case "sltu":
            if registers[r1] < registers[r2]:
                registers[rf] = 1
        case "xor":
            registers[rf] = registers[r1] ^ registers[r2]
            # print(registers[rf])
        case "sll":
            registers[rf] = registers[r1] * pow(2, registers[r2])
            # print(registers[rf])
        case "srl":
            registers[rf] = registers[r1] / pow(2, registers[r2])
            # print(registers[rf])
        case "or":
            registers[rf] = registers[r1] | registers[r2]
            # print(registers[rf])
        case "and":
            registers[rf] = registers[r1] & registers[r2]
            # print(registers[rf])


def matching_itype(inst, rf, r1, imm, pc, registers=registers, mem=mem):
    match inst:
        case "lw":
            print("###########################################")
            print(r1, imm)
            print(registers[r1], convertion(imm))
            print((hex(registers[r1] + convertion(imm))))
            try:
                if(hex(registers[r1] + convertion(imm)) in mem):
                    print("True")
                else:
                    print("False")
                registers[rf] = mem[(
                    hex(registers[r1] + convertion(imm)))]
                # print(hex(registers[r1] + convertion(imm)))
                print(hex_format(hex(registers[r1] + convertion(imm))))
                print(registers[rf])
                print("AAAAANNNNNUUUUUSSSSHHHHHHKKKKKKAAAAA")
            except KeyError:
                if(hex(registers[r1] + convertion(imm)) in mem):
                    print("True")
                else:
                    print("False")
                mem[(hex(registers[r1] + convertion(imm)))] = 0
                registers[rf] = mem[hex(registers[r1] + convertion(imm))]
            pc += 4
            # print(registers[rf])
        case "addi":
            registers[rf] = registers[r1] + convertion(imm)
            print(convertion(imm), registers[rf])
            pc += 4
            # print(registers[rf])
        case "sltiu":
            if registers[r1] < convertion(imm):
                registers[rf] = 1
            pc += 4
        case "jalr":
            registers[rf] = pc + 4
            pc = registers[r1] + convertion(imm)
            pc = int_to_32(pc)
            temp = pc[:-1] + "0"
            pc = binaryToDecimal(int(temp))

    # print(pc)
    return int(pc)


def matching_stype(inst, rf, r1, imm):
    match inst:
        case "sw":
            mem[hex(registers[r1] + convertion(imm))] = registers[rf]


def btype_s(my_array, registers, pc):
    imm = my_array[0:1] + my_array[26:27] + my_array[2:8] + my_array[21:25] + "0"
    rs2 = my_array[8:13]
    rs1 = my_array[13:18]
    func3 = my_array[18:21]
    match func3:
        case "000":
            if registers[rs1] == registers[rs2]:
                print(line)
                print("--------------------")
                print(imm)
                print(binaryToDecimal(int(imm)))
                print("--------------------")
                imm = binaryToDecimal(int(imm))
                print(pc)
                pc += imm
                print(pc)
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
        f.write(f"{hex_format(i)}:0b{int_to_32(mem[i])}")
        f.write("\n")
    print()
    f.close()


if __name__ == "__main__":
    # FIXME: file path
    file_path = os.path.abspath(
        "/home/ayush/Assembly-RISCV-CO2024/simulator/sim/input.txt"
    )
    # print(file_path)
    f = open("/home/ayush/Assembly-RISCV-CO2024/simulator/sim/output.txt", "w")
    f.close()

    with open(file_path, "r") as f:
        data = f.readlines()

    # print(data)

    halt = False
    while not halt:
        print(pc)
        # TODO: break loop logic

        line = data[pc // 4]
        # print(line)
        opcode = line[25:32]
        # print(opcode)

        if opcode == "0110011":
            print("R-type")
            funct7 = line[0:7]
            rs2 = line[7:12]
            rs1 = line[12:17]
            funct3 = line[17:20]
            rd = line[20:25]
            opcode = line[25:32]
            for keys, values in hash_map.items():
                if values[0] == opcode and values[1] == funct3 and values[2] == funct7:
                    # print(values[0], values[1], values[2])
                    matching_rtype(keys, rd, rs1, rs2)
            pc += 4

        elif opcode == "0000011" or opcode == "0010011" or opcode == "1100111":
            print("I-type")
            imm = line[0:12]
            print(line)
            print("----------------")
            print(imm)
            print(convertion(imm))
            rs1 = line[12:17]
            funct3 = line[17:20]
            rd = line[20:25]
            opcode = line[25:32]

            for keys, values in hash_map.items():
                if values[0] == opcode and values[1] == funct3:
                    # print(keys)
                    pc = matching_itype(keys, rd, rs1, imm, pc)

        elif opcode == "0100011":
            print("S-type")
            imm1 = line[0:7]
            rs1 = line[12:17]
            rd = line[7:12]
            funct3 = line[17:20]
            imm2 = line[20:25]
            imm = imm2 + imm1
            for keys, values in hash_map.items():
                if values[0] == opcode and values[1] == funct3:
                    # print(keys)
                    matching_stype(keys, rd, rs1, imm)
            print(pc)
            pc += 4

        elif opcode == "1100011":
            print("B-type")
            pc = btype_s(line, registers, pc)

        elif opcode == "0110111":
            print("U-type")
            # TODO: U-type logic
            opcode = input[25:31]
            if opcode == "0110111":
                immediate_bin = input[0:20]
                register_bin = input[20:25]
                final_bin = immediate_bin+"000000000000"
                dict[register_bin] = final_bin
            elif opcode == "0010111":
                programcount = binaryToDecimal(pc)
                immediate_bin = input[0:20]
                register_bin = input[20:25]
                final_bin = immediate_bin+"000000000000"
                final_num = binaryToDecimal(final_bin)
                final_num += programcount
                if (final_num >= 0):
                    dict[register_bin] = final_num
                else:
                    dict[register_bin] = final_num

        elif opcode == "1101111":
            print("J-type")
            # TODO: J-type logic
            reg = line[20:25]
            immediate = line[0]+line[1:9]+line[9:10]+line[10:20]+"0"
            registers[reg] = pc+4
            pc = binaryToDecimal(int(immediate))

        # elif opcode == "0000001":
        #     print("BONUS")
        #     funct7 = line[0:7]
        #     rs2 = line[7:12]
        #     rs1 = line[12:17]
        #     funct3 = line[17:20]
        #     rd = line[20:25]
        #     optcode = line[25:32]

        #     match inst:
        #         case "mul":
        #             registers[rd] = registers[rs1] * registers[rs2]
        #             print(registers[rd])
        #         case "rst":
        #             for i in registers:
        #                 registers[i] = 0
        #         case "halt":
        #             break
        #         case "rvrs":
        #             print(registers[rd])
        #             x = int_to_32(registers[rd])
        #             print(x)
        #             z = convertion2(x[::-1])
        #             print(z)
        #             registers[rs1] = z
        else:
            pc += 4
        if (pc // 4 >= len(data)):
            halt = True

        # write_reg()
        f = open("/home/ayush/Assembly-RISCV-CO2024/simulator/sim/output.txt", "a")
        f.write("0b")
        f.write(int_to_32(pc))
        f.write(" ")
        for i in registers:
            f.write("0b")
            f.write(str(int_to_32(registers[i])))
            f.write(" ")
        f.write("\n")
        f.close()
        # print(registers)

    write_mem()
    print(int_to_32(56))
