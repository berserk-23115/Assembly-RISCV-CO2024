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

pc=[0]

hash_map = {
    "lw": ["0000011", "010"],
    "addi": ["0010011", "000"],
    "sltiu": ["0010011", "011"],
    "jalr": ["1100111", "000"],
}

def convertion(y):
    if (y[0]=="0"):
        con= int(y,2)
        return con
    elif(y[0]=="1"):
        y = list(y)
        for i in range(12):
            if (y[i] == "0"):
                y[i] = "1"
            else:
                y[i] = "0"
        y = ''.join(y)
        con=int(y,2)
        con=con+1
        fin=con - (con*2)
        return fin

def matching(inst,rf,r1,imm):
    match inst:
        case "lw":
            registers[rf]=registers[r1]+convertion(imm)
            pc[0]+=4
            print(registers[rf])
        case "addi":
            registers[rf]=registers[r1]+convertion(imm)
            pc[0]+=4
            print(registers[rf])
        case "sltiu":
            if registers[r1]<convertion(imm):
                registers[rf]=1
                print(registers[rf])
            pc[0]+=4
        case "jalr":
            registers[rf]=pc[0]+4
            pc[0]=registers[rs1]+convertion(imm)


binary_number = str(input())

imm=binary_number[0:12]

rs1 = binary_number[12:17]

funct3 = binary_number[17:20]

rd = binary_number[20:25]

optcode = binary_number[25:32]


for keys, values in hash_map.items():
    if values[0] == optcode and values[1] == funct3:
        print(keys)
        matching(keys,rd,rs1,imm)

