f = open("test1.txt", "r")
g = open("output.txt", "r")

input = f.readlines()
output = g.readlines()

print(input == output)
