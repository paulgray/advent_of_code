#seq = list('637061')
seq = [3, 7]

first = 0
second = 1
created_count = 2

def print_recipes():
    s = ''
    for idx, score in enumerate(seq):
        if idx == first:
            s += '(' + str(score) + ')\t'
        elif idx == second:
            s += '[' + str(score) + ']\t'
        else:
            s += str(score) + '\t'

    print s

def create_new():
    global created_count
    combined = seq[first] + seq[second]
    if combined > 9:
        created_count += 1
        seq.append(1)
    created_count += 1
    seq.append(combined % 10)

def pick_new():
    global first, second

    first = (first + seq[first] + 1) % len(seq)
    second = (second + seq[second] + 1) % len(seq)

while created_count < 637061+10:
#    print_recipes()
    create_new()
    pick_new()

print ''.join(map(lambda x: str(x), seq[637061:637071]))
