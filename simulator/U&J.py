
def convertion(y):
        return  int(y,2)
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

def u_type(input, dict,pc):
    opcode = input[25:31]
    if opcode == "0110111":
        immediate_bin = input[0:20]
        register_bin = input[20:25]
        final_bin = immediate_bin+"000000000000"
        dict[register_bin]=final_bin
    elif opcode == "0010111":
        programcount = binaryToDecimal(pc)
        immediate_bin = input[0:20]
        register_bin = input[20:25]
        final_bin = immediate_bin+"000000000000"
        final_num = binaryToDecimal(final_bin)
        final_num+=programcount
        if(final_num>=0):
            dict[register_bin]= final_num
        else:
            dict[register_bin]= final_num


            

def j_type(input, dict,pc):
    register = input[20:25]
    programcount = pc
    immediate = input[0]+input[1:9]+input[9:10]+input[10:20]+"0"
    dict[register] = binaryToDecimal(programcount)+4
    pc = binaryToDecimal(immediate)

