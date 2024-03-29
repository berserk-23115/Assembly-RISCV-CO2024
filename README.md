
## Project Overview
This project involves developing software that converts the assembly code for RISCV32 ISA ( Instruction Set Architecture ) into machine code for RISC V-based processors and their simulation  in virtual machines. 
### About RISC-V
RISC V is an open-source Instruction Set Architecture developed by RISCV Foundation.   RISC V comes in three standard base versions with 32-bit, 64-bit, and 128-bit address spaces. It  is scalable and suitable for various devices, from embedded systems and microcontrollers to high-performance computing (HPC) applications.

### Features of Assembler
#### Error Detection : 

Invalid Instruction\
Invalid Register Declaration\
Missing Virtual Halt Instruction\
Virtual Halt Instruction not used as terminatory instruction\
Immediate passed is out of bounds


#### 

## Documentation

[Documentation](https://drive.google.com/file/d/1YbpAAX5xA26BjEXvGWXZL8_1BQg3h-29/view?usp=sharing)

## Project Usage : Linux

Select the folder in which you want to replicate this repository. 
```
$ git clone https://github.com/berserk-23115/Assembly-RISCV-CO2024/
```
```
$ cd Assembly-RISCV-CO2024
```
Change the permission of the script file ```Command.sh``` to make it executable
```
$ sudo chmod u+x Command.sh
```

Usage of ```Command.sh``` Script
```
$ ./command.sh
```
<span style = "color:red"> <em>WARNING :: EXECUTE THESE STEPS IN THE SAME ORDER !! </em></span>
1. This script originally checks for ```TEST```directory containing ```INPUT```,```OUTPUT```,```SIMULATE``` subdirectories. If not found, it would create them in the working directory in which this command is placed.
2. Place all the input text files in the ```INPUT``` Subdirectory.
3. Run the ```Command.sh``` file.
4. The ```OUTPUT``` and ```SIMULATE``` subdirectory should have the files naming as ```"$file_name_out.txt"``` and ```"$file_name_sim.txt"``` respectively.

## Prerequisites and System Requirements

Required System Requirements :\
OS:  MacOS, Linux, Windows\
HDD Space: 1 GB\
RAM: 2 GB


Installation of **RUST** programming Language \
Detailed Instructions :\
https://www.rust-lang.org/tools/install
## MacOS / Linux
```
curl https://sh.rustup.rs -sSf | sh
```
## Windows
Download the binary executable( Setup file ) from the below link :\
https://forge.rust-lang.org/infra/other-installation-methods.html \
\
Steps for installation :\
1.
## Windows Subsystem for Linux ( WSL )
To deploy this project, run

```bash
  
```


## Authors

- [Anushk Kumar](https://github.com/berserk-23115)
- [Ayush  Kumar Anand](https://github.com/ayushk-1801)
- [Ansh Goel](https://github.com/AnshG12)
- [Ayush Kitnawat](https://github.com/ayush-kitnawat-2023160)
