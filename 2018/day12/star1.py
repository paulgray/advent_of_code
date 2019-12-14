def final_score(generation):
    score = 10345
    per_gen = 78
    return (generation - 101) * per_gen + score

generations = 200
padding = 150

state = '.##..#.#..##..##..##...#####.#.....#..#..##.###.#.####......#.......#..###.#.#.##.#.#.###...##.###.#'
to_add = ''.join(['.' for _ in range(padding)])
state = to_add + state + to_add

ts = '''.##.# => #
##.#. => #
##... => #
#.... => .
.#..# => .
#.##. => .
.##.. => .
.#.## => .
###.. => .
..##. => #
##### => #
#...# => #
.#... => #
###.# => #
#.### => #
##..# => .
.###. => #
...## => .
..#.# => .
##.## => #
....# => .
#.#.# => #
#.#.. => .
.#### => .
...#. => #
..### => .
..#.. => #
..... => .
####. => .
#..## => #
.#.#. => .
#..#. => #'''
rules = dict(map(lambda line: tuple(line.split(' => ')), ts.split('\n')))
first_plant_idx = padding + 1

def get_score():
    score = 0
    offset = first_plant_idx - padding
    for idx, c in enumerate(list(state[first_plant_idx:])):
        if c == '#':
            score += idx + offset

    return score

new_score = prev_score = 0
for g in xrange(generations):
    new_state = state
    for i in range(first_plant_idx-5, len(state)-3):
        p = state[i:i+5]
        if p in rules:
            new_state = new_state[:i+2] + rules[p] + new_state[i+3:]
            if rules[p] == '#' and i < first_plant_idx:
                first_plant_idx = i
        else:
            new_state = new_state[:i+2] + '.' + new_state[i+3:]

    state = new_state
    prev_score = new_score
    new_score = get_score()
#    print str(g) + ' ' + str(new_score) + '\t' + str(new_score - prev_score) + '\t' + state[padding:]

print final_score(50000000000)
