m = [[], []]
pointer = 0
with open("test2") as f:
    for line in f:
        if line == "":
            continue

        m[0].append(list(line.strip()))
        m[1].append(list(line.strip()))

max_x = len(m[0][0])
max_y = len(m[0])

def next_pointer():
    global pointer
    pointer ^= 1

def print_map():
    global pointer
    for y in range(max_y):
        print ''.join(m[pointer][y])

def count_surroundings(x, y):
    empty = trees = lumbers = 0
    if x > 0 and y > 0:
        if m[pointer][x-1][y-1] == '.':
            empty += 1
        elif m[pointer][x-1][y-1] == '#':
            lumbers += 1
        elif m[pointer][x-1][y-1] == '|':
            trees += 1

    if x > 0:
        if m[pointer][x-1][y] == '.':
            empty += 1
        elif m[pointer][x-1][y] == '#':
            lumbers += 1
        elif m[pointer][x-1][y] == '|':
            trees += 1

    if x > 0 and y+1 < max_y:
        if m[pointer][x-1][y+1] == '.':
            empty += 1
        elif m[pointer][x-1][y+1] == '#':
            lumbers += 1
        elif m[pointer][x-1][y+1] == '|':
            trees += 1

    if y > 0:
        if m[pointer][x][y-1] == '.':
            empty += 1
        elif m[pointer][x][y-1] == '#':
            lumbers += 1
        elif m[pointer][x][y-1] == '|':
            trees += 1

    if y+1 < max_y:
        if m[pointer][x][y+1] == '.':
            empty += 1
        elif m[pointer][x][y+1] == '#':
            lumbers += 1
        elif m[pointer][x][y+1] == '|':
            trees += 1

    if x+1 < max_x and y > 0:
        if m[pointer][x+1][y-1] == '.':
            empty += 1
        elif m[pointer][x+1][y-1] == '#':
            lumbers += 1
        elif m[pointer][x+1][y-1] == '|':
            trees += 1

    if x+1 < max_x:
        if m[pointer][x+1][y] == '.':
            empty += 1
        elif m[pointer][x+1][y] == '#':
            lumbers += 1
        elif m[pointer][x+1][y] == '|':
            trees += 1

    if x+1 < max_x and y+1 < max_y:
        if m[pointer][x+1][y+1] == '.':
            empty += 1
        elif m[pointer][x+1][y+1] == '#':
            lumbers += 1
        elif m[pointer][x+1][y+1] == '|':
            trees += 1

    return [empty, trees, lumbers]

def tick():
    new_pointer = pointer ^ 1
    for y in range(max_y):
        for x in range(max_x):
            [empty, trees, lumbers] = count_surroundings(x, y)
            if m[pointer][x][y] == '.':
                ## An open acre will become filled with trees if three
                ## or more adjacent acres contained trees. Otherwise, nothing happens.
                if trees > 2:
                    m[new_pointer][x][y] = '|'
                else:
                    m[new_pointer][x][y] = '.'
            elif m[pointer][x][y] == '|':
                ## An acre filled with trees will become a lumberyard if three or
                ## more adjacent acres were lumberyards. Otherwise, nothing happens.
                if lumbers > 2:
                    m[new_pointer][x][y] = '#'
                else:
                    m[new_pointer][x][y] = '|'
            elif m[pointer][x][y] == '#':
                ## An acre containing a lumberyard will remain a lumberyard if it was
                ## adjacent to at least one other lumberyard and at least one acre
                ## containing trees. Otherwise, it becomes open.
                if lumbers > 0 and trees > 0:
                    m[new_pointer][x][y] = '#'
                else:
                    m[new_pointer][x][y] = '.'

    next_pointer()

def result():
    trees = lumbers = 0
    for y in range(max_y):
        for x in range(max_x):
            if m[pointer][x][y] == '#':
                lumbers += 1
            elif m[pointer][x][y] == '|':
                trees += 1

    return trees * lumbers

print_map()
generation = 0
current_result = prev_result = 0
while generation < 1000000000:
    tick()

#    print 'Generation ', generation
#    print_map()
#    print

    current_result = result()
    print 'Generation', generation, 'result', current_result, 'difference', prev_result - current_result
    prev_result = current_result

    generation += 1

print result()
