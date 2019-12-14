import re
import copy
import collections

OFFSET = 0

clay = []
with open("test2") as f:
    pattern1 = r'x=(\d+), y=(\d+)\.\.(\d+)'
    pattern2 = r'y=(\d+), x=(\d+)\.\.(\d+)'
    for line in f:
        if line == '':
            continue

        match = re.search(pattern1, line)
        if match != None:
            x, y1, y2 = match.groups()
            clay.append((int(x), int(x), int(y1), int(y2)))
        else:
            match = re.search(pattern2, line)
            if match != None:
                y, x1, x2 = match.groups()
                clay.append((int(x1), int(x2), int(y), int(y)))

max_x = max_y = 0
for (x1, x2, y1, y2) in clay:
    if x2 > max_x:
        max_x = x2
    if y2 > max_y:
        max_y = y2
max_x += 1
max_y += 1

m = [['.' for y in range(max_y)] for x in range(max_x)]

for (x1, x2, y1, y2) in clay:
    for x in range(x1, x2+1):
        for y in range(y1, y2+1):
            m[x][y] = '#'
m[500][0] = '+'
sources = collections.deque([[500, 0]])

def print_map():
    for y in range(max_y):
        s = ''
        for x in range(OFFSET, max_x):
            s += m[x][y]
        print s

def count_results():
    tiles = 0
    for y in range(max_y):
        for x in range(OFFSET, max_x):
            if m[x][y] == '~' or m[x][y] == '|':
                tiles += 1

    print tiles

def tick(source):
    [sx, sy] = source

    ## fall down
    x = sx
    y = sy+1
    while y < max_y and m[x][y] == '.':
        m[x][y] = '|'
        y += 1

    y -= 1
    source = [x, y]

    ## spill down
    lx = x-1
    rx = x+1
    spilling = False
    while lx >= 0 and m[lx][y] == '.' and not spilling and y+1 < max_y:
        if m[lx][y+1] == '.':
            spilling = True
        lx -= 1
    while rx < max_x and m[rx][y] == '.' and not spilling and y+1 < max_y:
        if m[rx][y+1] == '.':
            spilling = True
        rx += 1

    if spilling:
        lx = x-1
        rx = x+1
        while lx >= 0 and m[lx][y] == '.' and y+1 < max_y:
            m[lx][y] = '|'
            lx -= 1
            if m[lx][y+1] == '.':
                m[lx][y] = '|'
                s = [lx, y]
                sources.append(s)
                break
        while rx+1 < max_x and m[rx][y] == '.' and y+1 < max_y:
            m[rx][y] = '|'
            rx += 1
            if m[rx][y+1] == '.':
                m[rx][y] = '|'
                s = [rx, y]
                sources.append(s)
                break
            return

    if y+1 == max_y:
        return

    ## fill the reservoir
    lx = x-1
    rx = x+1
    filling = False
    while lx >= 0 and m[lx][y] == '.':
        m[lx][y] = '~'
        lx -= 1
        filling = True
    while rx < max_x and m[rx][y] == '.':
        m[rx][y] = '~'
        rx += 1
        filling = True

    if filling:
        m[x][y] = '~'
        source = [x, y-1]

    sources.append(source)

#print_map()
generation = 0
while sources:
    source = sources.popleft()
    tick(source)

    print
    print 'Generation ' + str(generation) + ' sources: ' + str(len(sources))
#    print_map()

    if len(sources) == 0:
        print 'Simulation done after generation ' + str(generation)
        count_results()
        break

    generation += 1
