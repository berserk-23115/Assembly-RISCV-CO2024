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
def signed_binaryToDecimal(binary,sign):
 
    decimal, i = 0, 0
    if(sign==0):
        while(binary != 0):
            dec = binary % 10
            decimal = decimal + dec * pow(2, i)
            binary = binary//10
            i += 1
        return(decimal)
    else:
        binary=binary%1000000000000
        while(binary != 0):
            dec = binary % 10
            decimal = decimal + dec * pow(2, i)
            binary = binary//10
            i += 1
        return(decimal*(-1))


    
def binaryToDecimal(binary):
 
    decimal, i = 0, 0
    while(binary != 0):
        dec = binary % 10
        decimal = decimal + dec * pow(2, i)
        binary = binary//10
        i += 1
    return(decimal)
def btype_s(my_array,registers,pc):
    imm=my_array[0:1]+my_array[26:27]+my_array[2:8]+my_array[21:25]+'0'
    rs2=my_array[8:13]
    rs1=my_array[13:18]
    func3=my_array[18:21]
    match func3:
        case '000':
            if(registers[rs1]==registers[rs2]):
                imm=binaryToDecimal(int(imm))
                pc+=imm
            else:
                pc+=4
        case '001':
            if(registers[rs1]!=registers[rs2]):
                imm=binaryToDecimal(int(imm))
                pc+=imm
            else:
                pc+=4
        case '100':
            if(registers[rs1]>=registers[rs2]):
                imm=binaryToDecimal(int(imm))
                pc+=imm
            else:
                pc+=4
        case '101':
            if(abs(registers[rs1])>=abs(registers[rs2])):
                imm=signed_binaryToDecimal(int(imm),int(imm[0]))
                pc+=imm
            else:
                pc+=4
        case '110':
            if(registers[rs1]<registers[rs2]):
                imm=binaryToDecimal(int(imm))
                pc+=imm
            else:
                pc+=4
        case '111':
            if(abs(registers[rs1])<abs(registers[rs2])):
                imm=binaryToDecimal(int(imm))
                pc+=imm
            else:
                pc+=4
    return(pc)

