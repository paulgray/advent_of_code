import copy

goblin_map = []
with open("test3") as f:
    for line in f.read().split("\n"):
        if len(line) == 0:
            continue
        goblin_map.append(line)

m_x = len(goblin_map[0])
m_y = len(goblin_map)

EMPTY  = 0
WALL   = 1
GOBLIN = 2
ELF    = 3
MARK   = 4

ATTACK_POWER = 3

m = [[EMPTY for x in range(m_x)] for y in range(m_y)]

## each creature has is an array:
## creature[0] = (x, y) - position on the map
## creature[1] = int(hp) - remaining hitpoints
## creature[2] = GOBLIN | ELF - type
goblins = []
elves = []
creatures = []
for y in range(m_y):
    for x in range(m_x):
        if goblin_map[x][y] == '#':
            m[x][y] = WALL
        elif goblin_map[x][y] == 'G':
            m[x][y] = GOBLIN
            goblin = [(x, y), 200, GOBLIN]
            goblins.append(goblin)
            creatures.append(goblin)
        elif goblin_map[x][y] == 'E':
            m[x][y] = ELF
            elf = [(x, y), 200, ELF]
            elves.append(elf)
            creatures.append(elf)

def print_map():
    for y in range(m_y):
        s = ''
        for x in range(m_x):
            if m[y][x] == EMPTY:
                s += '.'
            elif m[y][x] == ELF:
                s += 'E'
            elif m[y][x] == GOBLIN:
                s += 'G'
            elif m[y][x] == MARK:
                s += '!'
            else:
                s += '#'

        print s

    print

def print_creatures():
    for e in elves:
        (x, y) = e[0]
        print "Elf %d,%d with %d hp" % (x, y, e[1])

    for g in goblins:
        (x, y) = g[0]
        print "Goblin %d,%d with %d hp" % (x, y, g[1])

def alive(creatures):
    for c in creatures:
        if c[1] > 0:
            return True

    return False

def sort_creatures(creatures):
    creatures.sort(key = lambda tup: tup[0][::-1])

def identify_targets(attacker_type):
    target_type = ELF
    if attacker_type == ELF:
        target_type = GOBLIN

    targets = []
    for c in creatures:
        if c[1] > 0 and c[2] == target_type:
            targets.append(c)

    return targets

def identify_destinations(targets):
    destinations = []
    for t in targets:
        (x, y) = t[0]
        if m[x-1][y] == EMPTY:
            destinations.append((x-1, y))
        if m[x+1][y] == EMPTY:
            destinations.append((x+1, y))
        if m[x][y-1] == EMPTY:
            destinations.append((x, y-1))
        if m[x][y+1] == EMPTY:
            destinations.append((x, y+1))

    return destinations

def shortest_path((x0, y0), (x1, y1)):
    visited = set()
    visited.add((x0, y0))
    paths = [[(x0, y0)]]
    print 'Shortest path for %d,%d(%d) to %d,%d(%d)' % (x0, y0, m[x0][y0], x1, y1, m[x1][y1])
    while True:
        if len(paths) == 0:
            return None

        new_paths = []
        for p in paths:
            (x, y) = p[-1]
            if x == x1 and y == y1:

                prev0 = m[x0][y0]
                for (new_x, new_y) in p[1:]:
                    m[new_x][new_y] = MARK
                print_map()
                for (new_x, new_y) in p[1:]:
                    m[new_x][new_y] = EMPTY
                m[x0][y0] = prev0

                pos = p[1]
                print 'Next step: %d,%d' % (pos[0], pos[1])
                return (len(p), pos)

            for x2, y2 in [(0, -1), (-1, 0), (1, 0), (0, 1)]:
                new_pos = (x+x2, y+y2)
                if m[x+x2][y+y2] == EMPTY and new_pos not in visited:
                    new_p = copy.deepcopy(p)
                    new_p.append(new_pos)
                    new_paths.append(new_p)
                    visited.add(new_pos)

        paths = new_paths
        print paths

def find_path(c, destinations):
    costs = []
    for d in destinations:
        res = shortest_path(c[0], d)
        if res != None:
            (cost, pos) = res
            costs.append((cost, pos))

    costs.sort(key = lambda (cost, (x, y)): (cost, (x, y)))
    print 'Costs for moving: ' + str(costs)
    if len(costs) > 0:
        return costs[0]

    return None

def move(c):
    if identify_in_range(c[0], c[2]) != None:
        (x, y) = c[0]
        print "Creature %d,%d(%d) in range of an enemy, will not move!" % (x, y, c[2])
        return

    targets = identify_targets(c[2])
    destinations = identify_destinations(targets)
    next_step = find_path(c, destinations)
    if next_step != None:
        (cost, (x, y)) = next_step
        (prev_x, prev_y) = c[0]
        c[0] = (x, y)
        m[x][y] = c[2]
        m[prev_x][prev_y] = EMPTY

        print "Creature %d moves to %s" % (c[2], str(c[0]))

    print_map()

def find_creature(x, y, c):
    for creature in c:
        (x1, y1) = creature[0]
        if x1 == x and y1 == y:
            return creature

def identify_in_range(attacker_pos, attacker_type):
    (x, y) = attacker_pos
    defender_type = ELF
    if attacker_type == ELF:
        defender_type = GOBLIN

    targets = []
    for x2, y2 in [(0, -1), (-1, 0), (1, 0), (0, 1)]:
        if m[x+x2][y+y2] == defender_type:
            if defender_type == ELF:
                creature = find_creature(x+x2, y+y2, creatures)
                if creature[1] > 0:
                    targets.append(creature)

    if len(targets) > 0:
        targets.sort(key = lambda ((x, y), hp, t): (hp, (y, x)))
        return targets[0]

    return None

def attack(c):
    target = identify_in_range(c[0], c[2])
    if target != None:
        print 'Attacking ' + str(target)
        target[1] -= ATTACK_POWER

        if target[1] < 1:
            print 'Target dies!'
            (x, y) = target[0]
            m[x][y] = EMPTY

            print'Remaining creatures: ' + str(creatures)


print_map()

## main loop
turn = 1
while alive(goblins) > 0 and alive(elves) > 0:
#    raw_input('Press key to simulate')
    sort_creatures(creatures)
    for c in creatures:
        if c[1] < 1:
            continue

        move(c)
        attack(c)

    print 'Turn #' + str(turn)
    print_map()
    print_creatures()
    turn += 1

#    if turn > 20:
#        break

print creatures
