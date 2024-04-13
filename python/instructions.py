from bitstring import BitArray
def u_type(input, dict={}):
    opcode = input[25:31]
    if opcode == "0110111":
        immediate_bin = input[0:20]
        register_bin = input[20:25]
        final_bin = immediate_bin+"000000000000"
        dict[register_bin]=final_bin
    elif opcode == "0010111":
        programcount = BitArray(bin=dict['PC']).int
        immediate_bin = input[0:20]
        register_bin = input[20:25]
        final_bin = immediate_bin+"000000000000"
        final_num = BitArray(bin = final_bin).int
        final_num+=programcount
        if(final_num>=0):
            dict[register_bin]=

