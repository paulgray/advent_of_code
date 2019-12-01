import sys

sys.setrecursionlimit(10000)

GRID = 200
m = [['?' for x in range(GRID)] for y in range(GRID)]
px = py = GRID/2
r = ''
max_len = 0

with open("test1") as f:
    r = f.read().strip()[1:-1]

for y in range(GRID):
    for x in range(GRID):
        if (y % 2) == 0 and (x % 2) == 0:
            m[x][y] = '#'
        if (y % 2) == 1 and (x % 2) == 1:
            m[x][y] = '.'

def print_map():
    for y in range(GRID):
        s = ''
        for x in range(GRID):
            s += m[x][y]

        print s

    print

m[px][py] = 'X'

def plot(s):
    global px, py
    if s == '':
        return

    local_x = px
    local_y = py
    local_len = max_len
    c = s[0]

    if c == 'N':
        m[local_x][local_y-1] = '-'
        py -= 2
        plot(s[1:])
    elif c == 'S':
        m[local_x][local_y+1] = '-'
        py += 2
        plot(s[1:])
    elif c == 'E':
        m[local_x+1][local_y] = '|'
        px += 2
        plot(s[1:])
    elif c == 'W':
        m[local_x-1][local_y] = '|'
        px -= 2
        plot(s[1:])
    elif c == '(':
        alt_start = i = 1
        depth = 1
        alts = []
        while depth > 0:
            c = s[i]
            if c == '(':
                depth += 1
            elif c == ')' and depth == 1:
                alt = s[alt_start:i]
                alts.append(alt)
                alt_start = i+1
                depth -= 1
            elif c == ')':
                depth -= 1
            elif c == '|' and depth == 1:
                alt = s[alt_start:i]
                alts.append(alt)
                alt_start = i+1

            i += 1

#        print alts

        for alt in alts:
            px = local_x
            py = local_y
            plot(alt + s[i:])

def close_walls():
    for y in range(GRID):
        for x in range(GRID):
            if m[x][y] == '?':
                m[x][y] = '#'

print_map()
plot(r)
print_map()
close_walls()
print_map()
