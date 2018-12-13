import copy

m = []
with open("test1") as f:
    for line in f.read().split("\n"):
        if len(line) == 0:
            continue
        m.append(list(line))

m_y = len(m[0])
m_x = len(m)
collided = 0
tick_no = 0

def next_direction(direction):
    if direction == 'LEFT':
        return 'STRAIGHT'
    elif direction == 'STRAIGHT':
        return 'RIGHT'
    elif direction == 'RIGHT':
        return 'LEFT'

# cart list:
# cart[0] = (x, y) - position on the map
# cart[1] = 'UP' | 'DOWN' | 'LEFT' | 'RIGHT' - direction cart is going
# cart[2] = 'LEFT' | 'STRAIGHT' | 'RIGHT' - next turn
# cart[3] = True | False - has collided
def find_carts():
    c = []
    for i in range(m_x):
        for j in range(m_y):
            if m[i][j] == '^':
                c.append([(i, j), 'UP', 'LEFT', False])
                m[i][j] = '|'
            elif m[i][j] == '>':
                c.append([(i, j), 'RIGHT', 'LEFT', False])
                m[i][j] = '-'
            elif m[i][j] == '<':
                c.append([(i, j), 'LEFT', 'LEFT', False])
                m[i][j] = '-'
            elif m[i][j] == 'v':
                c.append([(i, j), 'DOWN', 'LEFT', False])
                m[i][j] = '|'

    return c

def sort_carts(c):
    return sorted(c, key = lambda tup: tup[0][::-1])

def find_collision(c):
    global collided

    for i in range(len(c)-1):
        if c[i][3]:
            continue

        for j in range(i+1, len(c)):
            if c[j][3]:
                continue

            if c[i][0] == c[j][0]:
                print 'Tick #' + str(tick_no) + ' - collision at ' + str(c[i][0])
#                for cart in c:
#                    print cart
#                print

                c[i][3] = True
                c[j][3] = True
                collided += 2

def turn(cart):
    direction = cart[1]
    next_turn = cart[2]
    if direction == 'UP':
        if next_turn == 'LEFT':
            cart[1] = 'LEFT'
        elif next_turn == 'RIGHT':
            cart[1] = 'RIGHT'
        else:
            cart[1] = 'UP'
    elif direction == 'DOWN':
        if next_turn == 'LEFT':
            cart[1] = 'RIGHT'
        elif next_turn == 'RIGHT':
            cart[1] = 'LEFT'
        else:
            cart[1] = 'DOWN'
    elif direction == 'LEFT':
        if next_turn == 'LEFT':
            cart[1] = 'DOWN'
        elif next_turn == 'RIGHT':
            cart[1] = 'UP'
        else:
            cart[1] = 'LEFT'
    else:
        if next_turn == 'LEFT':
            cart[1] = 'UP'
        elif next_turn == 'RIGHT':
            cart[1] = 'DOWN'
        else:
            cart[1] = 'RIGHT'

    cart[2] = next_direction(next_turn)

    return cart

def curve(cart, d):
    direction = cart[1]
    if d == '/':
        if direction == 'UP':
            cart[1] = 'RIGHT'
        elif direction == 'DOWN':
            cart[1] = 'LEFT'
        elif direction == 'LEFT':
            cart[1] = 'DOWN'
        else:
            cart[1] = 'UP'
    elif d == '\\':
        if direction == 'UP':
            cart[1] = 'LEFT'
        elif direction == 'DOWN':
            cart[1] = 'RIGHT'
        elif direction == 'LEFT':
            cart[1] = 'UP'
        else:
            cart[1] = 'DOWN'

    return cart

def tick(c):
    for cart in c:
        (x, y) = cart[0]

        if m[x][y] == '+':
            ## intersection
            cart = turn(cart)
        elif m[x][y] == '/' or m[x][y] == '\\':
            cart = curve(cart, m[x][y])

        if cart[1] == 'UP':
            x -= 1
        elif cart[1] == 'DOWN':
            x += 1
        elif cart[1] == 'LEFT':
            y -= 1
        else:
            y += 1

        cart[0] = (x, y)

        find_collision(c)

    return c

def print_map(c):
    m_c = copy.deepcopy(m)

    for cart in carts:
        print cart

    print

    for cart in c:
        (x, y) = cart[0]
        direction = cart[1]

        if direction == 'LEFT':
            m_c[x][y] = '<'
        elif direction == 'RIGHT':
            m_c[x][y] = '>'
        elif direction == 'UP':
            m_c[x][y] = '^'
        else:
            m_c[x][y] = 'v'

    for line in m_c:
        print ''.join(line)

    print
    print

carts = sort_carts(find_carts())
while len(carts) > collided + 1:
#    print 'Tick #' + str(i)

    carts = sort_carts(tick(carts))
#    print_map(carts)
    tick_no += 1

for c in carts:
    print c
