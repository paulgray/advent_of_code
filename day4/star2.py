import re

actions = []
with open("test1") as f:
    pattern = r'\[(.+)\] (.+)'
    for line in f:
        match = re.search(pattern, line.strip())
        (ts, action) = match.groups()
        actions.append((ts, action, ))

actions.sort(key = lambda tup: tup[0])

minutes = [{} for x in range(59)]
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
        wakes_up = int(wakes_up_str)

        for minute in range(asleep_since, wakes_up):
            guards = minutes[minute]
            if current in guards:
                guards[current] += 1
            else:
                guards[current] = 1
    else:
        match = re.search(change_guard_pattern, action)
        (current, ) = match.groups()

max_guard_id = ''
max_count = 0
max_minute = 0
for idx, minute in enumerate(minutes):
    for guard_id, count in minute.items():
        if count > max_count:
            max_guard_id = guard_id
            max_minute = idx
            max_count = count

print int(max_guard_id) * max_minute
