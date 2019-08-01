try:
    n = int(input("N?"))
    f = open("input.txt", "w")

    for i in range(n):
        for j in range(n):
            f.write("0")
        f.write("\n")
    f.close()
    
except:
    print("Input needs to be an Integer")
