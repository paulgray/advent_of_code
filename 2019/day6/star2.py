import re

def distance(x1, y1, coord):
    (x2, y2) = coord
    return abs(x1 - x2) + abs(y1 - y2)

## read coords
coords = []
with open("test1") as f:
    pattern = r'(\d+), (\d+)'
    for line in f:
        match = re.search(pattern, line)
        (x, y) = match.groups()
        coords.append((int(x), int(y), ))

## normalize (let's move all coords as close to (0, 0) as possible
min_x = min_y = 9999
max_x = max_y = 0
for (x, y) in coords:
    if x < min_x:
        min_x = x
    if y < min_y:
        min_y = y
    if x > max_x:
        max_x = x
    if y > max_y:
        max_y = y
coords = map(lambda (x, y): (x-min_x, y-min_y), coords)
max_x -= min_x
max_y -= min_y

## calculate the distance between each point on the map to all other coords
MAX_DISTANCE = 10000
area_size = 0
for x in range(max_x):
    for y in range(max_y):
        d = sum([distance(x, y, coord) for coord in coords])
        if d < MAX_DISTANCE:
            area_size += 1

print area_size
