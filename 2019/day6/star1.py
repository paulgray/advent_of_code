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

## build a distance map
areas = [[-1 for x in range(max_y)] for y in range(max_x)]

for x in range(max_x):
    for y in range(max_y):
        min_idx = 0
        min_distance = 9999
        for (idx, coord) in enumerate(coords):
            d = distance(x, y, coord)
            if d < min_distance:
                min_distance = d
                min_idx = idx
            elif d == min_distance:
                min_idx = -1

        areas[x][y] = min_idx

## calculate area sizes
sizes = {}
for x in range(max_x):
    for y in range(max_y):
        code = areas[x][y]
        if code != -1:
            if code in sizes:
                sizes[code] += 1
            else:
                sizes[code] = 1

## filter out infinites
for x in range(max_x):
    code = areas[x][0]
    sizes[code] = 0

    code = areas[x][max_y-1]
    sizes[code] = 0

for y in range(max_y):
    code = areas[0][y]
    sizes[code] = 0

    code = areas[max_x-1][y]
    sizes[code] = 0

max_area = max(sizes.values())

print max_area
