import re

steps = []
with open("test1") as f:
    pattern = r'Step (.) must be finished before step (.) can begin'
    for line in f:
        match = re.search(pattern, line)
        (after, before) = match.groups()
        steps.append((after, before))

s = ''
completed = set()
remaining = set()

for (after, before) in steps:
    remaining.add(after)
    remaining.add(before)

while len(remaining) > 0:
    to_add = set()
    for step in remaining:
        satisfied = True
        for (after, before) in steps:
            if before == step:
                if after not in completed:
                    satisfied = False
                    break

        if satisfied:
            to_add.add(step)

    to_add_step = list(to_add)
    to_add_step.sort()
    s += to_add_step[0]

    remaining.remove(to_add_step[0])
    completed.add(to_add_step[0])

print s
