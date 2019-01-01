import copy, re

#register = [1, 0, 0, 0, 0, 0]
register = [10551329, 10551328, 0, 1, 3, 10551329]
ip = 0

def addr(a, b, c):
    register[c] = register[a] + register[b]

def addi(a, b, c):
    register[c] = register[a] + b

def mulr(a, b, c):
    register[c] = register[a] * register[b]

def muli(a, b, c):
    register[c] = register[a] * b

def banr(a, b, c):
    register[c] = register[a] & register[b]

def bani(a, b, c):
    register[c] = register[a] & b

def borr(a, b, c):
    register[c] = register[a] | register[b]

def bori(a, b, c):
    register[c] = register[a] | b

def setr(a, b, c):
    register[c] = register[a]

def seti(a, b, c):
    register[c] = a

def gtir(a, b, c):
    if a > register[b]:
        register[c] = 1
    else:
        register[c] = 0

def gtri(a, b, c):
    if register[a] > b:
        register[c] = 1
    else:
        register[c] = 0

def gtrr(a, b, c):
    if register[a] > register[b]:
        register[c] = 1
    else:
        register[c] = 0

def eqir(a, b, c):
    if a == register[b]:
        register[c] = 1
    else:
        register[c] = 0

def eqri(a, b, c):
    if register[a] == b:
        register[c] = 1
    else:
        register[c] = 0

def eqrr(a, b, c):
    if register[a] == register[b]:
        register[c] = 1
    else:
        register[c] = 0

instructions = []
with open("test2") as f:
    lines = f.read().split("\n")
    ip = int(lines[0].strip().split(" ")[1])

    possibles = globals().copy()
    possibles.update(locals())

    pattern = r'([a-z]+) (\d+) (\d+) (\d+)'
    for g in lines[1:]:
        match = re.search(pattern, g)
        if match != None:
            (instr, a, b, c) = match.groups()
            func = possibles.get(instr)
            instructions.append((func, int(a), int(b), int(c)))

count = 0
while True:
    i = register[ip]
    (instr, a, b, c) = instructions[i]
#    print 'ip=', register[ip], register, instructions[i]
    instr(a, b, c)
    register[ip] += 1
    count += 1
    print register

    if register[ip] >= len(instructions):
        break

    if (count % 100000) == 0:
        print count

print register[0]
