doubles = 0
triples = 0

with open("test1") as f:
    for line in f:
        letters = {}
        for letter in list(line):
            if letters.has_key(letter):
                letters[letter] += 1
            else:
                letters[letter] = 1

        d = t = 0
        for count in letters.values():
            if count == 2:
                d += 1
            elif count == 3:
                t += 1

        if d > 0:
            doubles += 1
        if t > 0:
            triples += 1

print doubles * triples
