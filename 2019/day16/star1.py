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
op_codes = [None for x in range(len(ops))]

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

found = 0
while found != 16:
    for (before_reg, instruction, after_reg) in instructions:
        (op_no, a, b, c) = instruction
        hits = 0
        op_ptr = None
        for op in ops:
            register = copy.deepcopy(before_reg)
            op(a, b, c)
            if register == after_reg and op not in op_codes:
                hits += 1
                op_ptr = op

        if hits == 1:
            print 'Found op! Code %d is %s' % (op_no, op_ptr.__name__)
            op_codes[op_no] = op_ptr
            found += 1

register = [0, 0, 0, 0]
with open("test3") as f:
    pattern = r'(\d+) (\d+) (\d+) (\d+)'
    for g in f.read().split("\n"):
        match = re.search(pattern, g)
        (op_no, a, b, c) = match.groups()
        op = op_codes[int(op_no)]
        op(int(a), int(b), int(c))

print register[0]
