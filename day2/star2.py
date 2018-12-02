def distance(w1, w2):
    d = 0
    for idx, letter in enumerate(w1):
        if letter != w2[idx]:
            d += 1

    return d

def print_same(w1, w2):
    s = ""
    for idx, letter in enumerate(w1):
        if letter == w2[idx]:
            s += letter

    print s

codes = []
with open("test1") as f:
    for line in f:
        codes.append(line.strip())

for i in range(0, len(codes)):
    code = codes[i]
    for j in range(i+1, len(codes)):
        comp = codes[j]

#        print code + " " + comp + " => " + str(distance(code, comp))

        if distance(code, comp) == 1:
            print_same(code, comp)
            break
