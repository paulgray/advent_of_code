import copy, re

register = [0, 0, 0, 0]

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

ops = [addr, addi,
           mulr, muli,
           banr, bani,
           borr, bori,
           setr, seti,
           gtir, gtri, gtrr,
           eqir, eqri, eqrr]

instructions = []
with open("test1") as f:
    pattern = r'Before: \[(\d+), (\d+), (\d+), (\d+)\]\n(\d+) (\d+) (\d+) (\d+)\nAfter:  \[(\d+), (\d+), (\d+), (\d+)\]'
    for g in f. read().split("\n\n"):
        match = re.search(pattern, g)
        if match != None:
            (b1, b2, b3, b4, op, a, b, c, a1, a2, a3, a4) = match.groups()
            before_reg = [int(b1), int(b2), int(b3), int(b4)]
            after_reg = [int(a1), int(a2), int(a3), int(a4)]
            operation = [int(op), int(a), int(b), int(c)]
            instructions.append((before_reg, operation, after_reg))

result = 0
for (before_reg, instruction, after_reg) in instructions:
    (op_no, a, b, c) = instruction
    hits = 0
    for op in ops:
#        print 'Before ' + op.__name__ + ' reg: ' + str(before_reg) + 'operands: ' + str((a, b, c))
        register = copy.deepcopy(before_reg)
        op(a, b, c)
#        print 'After ' + op.__name__ + ' reg: ' + str(register) + ' expected reg: ' + str(after_reg)
        if register == after_reg:
#            print 'Its a match! ' + op.__name__
            hits += 1
            if hits >= 3:
                break

    if hits >= 3:
#        print 'Instruction ' + str(instruction) + ' behaves like ' + str(hits) + ' opscodes'
        result += 1

print result
