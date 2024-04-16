registers = {
    "00000": 1,
    "00001": 0,
    "00010": 10,
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
    "mul": ["0000001", "000", "0000000"],
    "rst": ["0000001", "001", "0000000"],
    "halt": ["0000001", "010", "0000000"],
    "rvrs": ["0000001", "011", "0000000"],
}
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
        y[0]="1"
        y = ''.join(y)
        return y
        print(y)
def convertion(y):
        return  int(y,2)

def matching(inst, rf, r1, r2):
    match inst:
        case "mul":
            registers[rf] = registers[r1] * registers[r2]
            print(registers[rf])
        case "rst":
            for i in registers:
                registers[i]=0
        case "halt":
            pass#need to exit the code(halt)
        case "rvrs":
            print(registers[rf])
            x = int_to_32(registers[rf])
            print(x)
            z = convertion(x[::-1])
            print(z)
            registers[r1] = z


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
        matching(keys,rd,rs1,rs2)

