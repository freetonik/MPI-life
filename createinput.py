from random import randrange

try:
    n = int(input("N?"))
    f = open("input.txt", "w")

    f.write(str(n)+ " 800 800\n")
    for i in range(n):
        for j in range(n):
            f.write(str(randrange(2)))
        f.write("\n")
    f.close()

except:
    print("Input needs to be an Integer")
