import re

workers_no = 5
s = ''
workers = [(0, '.') for x in range(workers_no)]
second = 0

completed = set()
remaining = set()
in_progress = set()

def cost(letter):
    return ord(letter) - ord('A') + 1

def expire_workers():
    global s
    to_add = []
    for idx, (done_second, current_job) in enumerate(workers):
        if done_second == second and current_job != '.':
            to_add.append(current_job)
            in_progress.remove(current_job)
            completed.add(current_job)
            workers[idx] = (second, '.')

    to_add.sort()
    s += ''.join(to_add)

def workers_available():
    for (done_second, current_job) in workers:
        if current_job == '.':
            return True

    return False

def assign_work(seconds, job):
    for idx, (done_second, current_job) in enumerate(workers):
        if current_job == '.':
            workers[idx] = (seconds, job)
            remaining.remove(job)
            in_progress.add(job)
            break

steps = []
with open("test1") as f:
    pattern = r'Step (.) must be finished before step (.) can begin'
    for line in f:
        match = re.search(pattern, line)
        (after, before) = match.groups()
        steps.append((after, before))

for (after, before) in steps:
    remaining.add(after)
    remaining.add(before)

while len(remaining) > 0 or len(in_progress) > 0:
    expire_workers()
    if workers_available() and len(remaining) > 0:
        while workers_available() and len(remaining) > 0:
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

            if len(to_add) == 0:
                break

            to_add_step = list(to_add)
            to_add_step.sort()
            finished = cost(to_add_step[0]) + second + 60

            assign_work(finished, to_add_step[0])

    second += 1

print second-1
