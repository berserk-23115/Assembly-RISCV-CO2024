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
    "11111": 0
}

hash_map = {
    "add": ["0011011", "000", "0000000"],
    "sub": ["0011011", "000", "0100000"],
    "sll": ["0011011", "001", "0000000"],
    "slt": ["0011011", "010", "0000000"],
    "sltu": ["0011011", "011", "0000000"],
    "xor": ["0011011", "100", "0000000"],
    "srl": ["0011011", "101", "0000000"],
    "or": ["0011011", "110", "0000000"],
    "and": ["0011011", "111", "0000000"]
}

def matching(inst,rf,r1,r2):
    match inst:
        case "add":
            registers[rf]=registers[r1]+registers[r2]
            print(registers[rf])
        case "sub":
            registers[rf]=registers[r1]-registers[r2]
            print(registers[rf])
        case "slt":
            if registers[r1]<registers[r2]:
                registers[rf]=1
        case "sltu":
            if registers[r1]<registers[r2]:
                registers[rf]=1
        case "xor":
            registers[rf]=registers[r1]^registers[r2]
            print(registers[rf])
        case "sll":
            registers[rf]=registers[r1]*pow(2,registers[r2])
            print(registers[rf])
        case "slr":
            registers[rf]=registers[r1]/pow(2,registers[r2])
            print(registers[rf])
        case "or":
            registers[rf]=registers[r1]|registers[r2]
            print(registers[rf])
        case "and":
            registers[rf]=registers[r1]&registers[r2]
            print(registers[rf])


binary_number = str(input())
funct7 = binary_number[0:7]
rs2 = binary_number[7:12]
print(rs2)
rs1 = binary_number[12:17]
print(rs1)
funct3 = binary_number[17:20]
rd = binary_number[20:25]
print(rd)
optcode = binary_number[25:32]

for keys, values in hash_map.items():
    if values[0] == optcode and values[1] == funct3 and values[2] == funct7:
        print(keys)
        matching(keys, rd, rs1, rs2)

# print(optcode, funct3, funct7)
# for i in hash_map:
#     print(hash_map[i][0], hash_map[i][1], hash_map[i][2])
#     if hash_map[i][0] == optcode and hash_map[i][1] == funct3 and hash_map[i][2] == funct7:
#         print(i)
#         matching(i, rd, rs1, rs2)
    
