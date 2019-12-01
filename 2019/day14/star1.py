seq = [3, 7]
seq_str = '37'
pattern = '637061'
#pattern = '59414'
pattern_len = len(pattern)

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
    global created_count, seq_str
    combined = seq[first] + seq[second]
    if combined > 9:
        created_count += 1
        seq.append(1)
        seq_str += '1'
    created_count += 1
    seq.append(combined % 10)
    seq_str += str(combined % 10)

def pick_new():
    global first, second

    first = (first + seq[first] + 1) % len(seq)
    second = (second + seq[second] + 1) % len(seq)

def find_pattern():
    if seq_str[-pattern_len:] == pattern:
        return len(seq_str) - pattern_len
    elif seq_str[-pattern_len:-1] == pattern:
        return len(seq_str) - pattern_len
    return -1

iterations = 0
while find_pattern() == -1:
#    print_recipes()
    create_new()
    pick_new()

    if (iterations % 10000) == 0:
        print iterations

    iterations += 1

print find_pattern()
