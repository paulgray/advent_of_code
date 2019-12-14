import re

actions = []
with open("test1") as f:
    pattern = r'\[(.+)\] (.+)'
    for line in f:
        match = re.search(pattern, line.strip())
        (ts, action) = match.groups()
        actions.append((ts, action, ))

actions.sort(key = lambda tup: tup[0])

guards = {}
minutes_slept = {}
current = ''
asleep_since = 0
change_guard_pattern = r'Guard #(\d+) .*'
minutes_pattern = r'....-..-.. ..:(\d\d)'
for (ts, action, ) in actions:
    if action == 'falls asleep':
        match = re.search(minutes_pattern, ts)
        (asleep_since_str, ) = match.groups()
        asleep_since = int(asleep_since_str)
    elif action == 'wakes up':
        match = re.search(minutes_pattern, ts)
        (wakes_up_str, ) = match.groups()
        total_time = int(wakes_up_str) - asleep_since

        if current in guards:
            guards[current] += total_time

            for minute in range(asleep_since, int(wakes_up_str)):
                minutes_slept[current][minute] += 1
        else:
            guards[current] = total_time
            minutes_slept[current] = [0 for x in range(59)]

            for minute in range(asleep_since, int(wakes_up_str)):
                minutes_slept[current][minute] = 1
    else:
        match = re.search(change_guard_pattern, action)
        (current, ) = match.groups()

max_guard_id = ''
max_total_time = 0
for guard_id, total_time in guards.items():
    if total_time > max_total_time:
        max_total_time = total_time
        max_guard_id = guard_id

max_minutes_no = 0
max_minutes_slept = 0
for idx, count in enumerate(minutes_slept[max_guard_id]):
    if count > max_minutes_slept:
        max_minutes_slept = count
        max_minutes_no = idx

print int(max_guard_id) * max_minutes_no
