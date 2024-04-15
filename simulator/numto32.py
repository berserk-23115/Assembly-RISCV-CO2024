x = int(input())

if x >= 0:
    y = format(x, '032b')
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
    print(y)
