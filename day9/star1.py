def insert_marble(turn, current_idx, ring):
    idx = (current_idx + 1) % len(ring) + 1
    ring.insert(idx, turn)

    return (idx, ring)

def remove_marble(current_idx, ring, current_player, score):
    idx = (current_idx - 7) % len(ring)
    marble = ring.pop(idx)
    score[current_player] += marble

    return (idx, ring, score)

def print_ring(player, idx, ring):
    s = '[' + str(player+1) + ']\t'
    for i, marble in enumerate(ring):
        if idx == i:
            s += '(' + str(marble) + ')'
        else:
            s += str(marble)
        s += '\t'

    print s

MAX_POINTS = 7173000
PLAYERS_COUNT = 464
#MAX_POINTS = 1618
#PLAYERS_COUNT = 10

scores = [0 for i in range(PLAYERS_COUNT)]
ring = [0]
current_player = current_index = current_marble = 0
#print_ring(current_player, current_index, ring)
for turn in range(1, MAX_POINTS+1):
    if (turn % 23) == 0:
        scores[current_player] += turn
        (current_index, ring, scores) = remove_marble(current_index, ring, current_player, scores)
    else:
        (current_index, ring) = insert_marble(turn, current_index, ring)
        current_marble = turn

    #print_ring(current_player, current_index, ring)
    if (turn % 20000) == 0:
        print 'Turn ' + str(turn)

    current_player = (current_player+1) % PLAYERS_COUNT

print str(max(scores))
